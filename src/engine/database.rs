pub mod engine {
    use crate::storage::column::generic::{Uncompressed, ColumnInterface, Column};
    use crate::storage::table::table::*;
    use crate::sqlscript::parser::parser::*;
    use crate::sqlscript::types::types::*;
    use super::super::script::env::*;
    use super::super::script::engine::*;
    use std::{env, error::Error, fs::File, process, ffi::OsString, rc::Rc};

    macro_rules! handle{
        ($e:expr) => {
            (match $e { Ok(v) => v, Err(s) => return QueryResult::Error(s) })
        }
    }

    pub enum QueryResult {
        Table(Table),
        Value(Val),
        Error(String),
        Success(String),
        Exit
    }

    pub struct Database {
        tables: Vec<Table>,
        table_names: Vec<String>,
        constants: Vec<(String, Val)>,
        calculated: Vec<Vec<Option<Expr>>>
    }
    impl Database {
        fn insert(&mut self, table_name: &String, fields: &Option<Vec<String>>, values: &Vec<Rc<Expr>>) -> QueryResult {
            // Get referenced table
            let table_idx = handle!(self.get_table_index(table_name));
            let table = &self.tables[table_idx];
            // Create environment
            let mut default_env = Environment::new();
            // Evaluate given values
            let mut values_insert: Vec<Val> = Vec::new();
            let mut i: usize = 0;
            for field_name in table.get_headers() {
                // Check if is calculated
                match &self.calculated[table_idx][i] {
                    Some(expr) => {
                        // Environment
                        let mut env = self.default_environment();
                        // Push already-added values onto environment
                        for j in 0..i {
                            env.push(&table.get_headers()[j], &values_insert[j]);
                        }
                        // Evaluate
                        let val = handle!(eval(expr, &mut env));
                        // Insert val
                        values_insert.push(val);
                    },
                    None => {
                        match fields {
                            Some(inserted_fields) => {
                                // Get index of field name in inserted fields
                                match inserted_fields.iter().position(|r| { r == field_name }) {
                                    Some(idx) => {
                                        // Evaluate next value
                                        let val = handle!(eval(values[idx].as_ref(), &mut default_env));
                                        // Push onto values_insert
                                        values_insert.push(val);
                                    },
                                    None => values_insert.push(Val::NullVal)
                                }
                            },
                            None => {
                                // Evaluate next value
                                let val = handle!(eval(values[i].as_ref(), &mut default_env));
                                // Push onto values_insert
                                values_insert.push(val);
                            }
                        }
                    }
                }
                // Increment i
                i += 1;
            }
            // Call insert and update
            self.insert_and_update(table_name, table_idx, values_insert)
        }
        fn insert_and_update(&mut self, table_name: &String, table_idx: usize, values_insert: Vec<Val>) -> QueryResult {
            // Borrow table as immutable
            let table = &self.tables[table_idx];
            // Is this the first row?
            let first_row = table.len() == 0;
            // Calculate aggregates
            let mut ag_vals = Vec::new();
            for ag in table.get_aggregates() {
                // Are you given an initial aggregate?
                let has_init = match &ag.3 {
                    Some(_) => true,
                    _ => false,
                };
                // Environment
                let mut env = self.default_environment();
                // Add new row to environment
                for i in 0..values_insert.len() {
                    env.push(&table.get_headers()[i], &values_insert[i]);
                }
                // Add current value to environment, unless is first row and has init
                if !(first_row && has_init) {
                    env.push(&"current".to_string(), &ag.1);
                }
                // Evaluate
                let val = match first_row {
                    true => match &ag.3 {
                        Some(e1) => handle!(eval(e1, &mut env)),
                        None => handle!(eval(&ag.2, &mut env))
                    },
                    false => handle!(eval(&ag.2, &mut env))
                };
                // Push
                ag_vals.push(val);
            }
            // Calculate computations
            let mut cmp_vals = Vec::new();
            for cmp in table.get_computations() {
                // Environment
                let mut env = self.default_environment();
                let mut i = 0;
                for ag in table.get_aggregates() {
                    env.push(&ag.0, &ag_vals[i]);
                    i += 1;
                }
                // Evaluate
                cmp_vals.push(handle!(eval(&cmp.2, &mut env)));
            }
            // Borrow table as mutable
            let table = &mut self.tables[table_idx];
            // Add row to table
            handle!(table.add_row(values_insert));
            // Add aggregates
            handle!(table.update_aggregates(&ag_vals));
            // Add computations
            handle!(table.update_computations(&cmp_vals));
            // Return
            QueryResult::Success("Insert on ".to_string() + table_name)
        }
        fn create_table(&mut self, table_name: &String, schema: &Vec<(String, ColType, Option<CompressType>)>) -> QueryResult {
            // Check that table doesn't already exist
            if self.table_names.contains(table_name) { panic!("Table already exists") }
            // Push table name
            self.table_names.push(table_name.clone());
            // Create new table
            self.tables.push(Table::new());
            // Get index of new table
            let idx: usize = self.tables.len() - 1;
            // Create new calculated vector
            self.calculated.push(Vec::new());
            // Add schema to new table, mark all columns as not calculated
            for schema_item in schema {
                let ctype = match schema_item.2 {
                    Some(x) => x,
                    _ => CompressType::Uncompressed
                };
                handle!(self.tables[idx].add_column(&schema_item.0, schema_item.1, ctype));
                self.calculated[idx].push(None)
            }
            QueryResult::Success("Created table ".to_string() + table_name)
        }
        fn select(&mut self, fields: &Option<Vec<String>>, table_name: &String, where_: &Option<Expr>, sort_by: &Option<(String, SortType)>, limit: &Option<Expr>) -> QueryResult {
            // Get referenced table
            let table_idx = self.table_names.iter().position(|r| *r == *table_name).unwrap();
            let table = &self.tables[table_idx];
            // Vector of added rows
            let mut added_rows: Vec<Vec<Val>> = Vec::new();
            let mut full_rows: Vec<(Vec<Val>, usize)> = Vec::new();
            // Iterate through each row in the table
            let mut i: usize = 0;
            for row in table.iter() {
                // New row
                let mut new_row: Vec<Val> = Vec::new();
                // Environment in which to evaluate row
                let mut env = self.default_environment();
                // Add aggregates into environment
                table.push_aggregates(&mut env);
                // Add all fields to environment
                for field in table.get_headers() {
                    let idx = handle!(table.header_idx(field));
                    env.push(field, &row[idx]);
                }
                // Add items to new row
                match fields {
                    Some(v) => {
                        for field in v {
                            let idx = handle!(table.header_idx(field));
                            new_row.push(row[idx].clone());
                        }
                    },
                    None => {
                        for field in table.get_headers() {
                            let idx = handle!(table.header_idx(field));
                            new_row.push(row[idx].clone());
                        }
                    }
                }
                // Evaluate where clause, convert to bool
                let should_add = match where_ {
                    Some(expr) => handle!(eval_bool(expr, &mut env)),
                    None => true
                };
                // Push to new table if should add
                if should_add {
                    added_rows.push(new_row);
                    // Only add to full rows if sorting
                    match sort_by {
                        Some(_) => {
                            full_rows.push((row.clone(), i));
                            // Increment i
                            i += 1;
                        },
                        None => ()
                    };
                }
                
            };
            // If sorting, sort full rows by sort_by header
            match sort_by {
                Some(s) => {
                    let col_idx = handle!(table.header_idx(&s.0));
                    match &s.1 {
                        SortType::Ascending => full_rows.sort_by(|a, b| eval_ordering(&(a.0)[col_idx], &(b.0)[col_idx])),
                        SortType::Descending => full_rows.sort_by(|a, b| eval_ordering_desc(&(a.0)[col_idx], &(b.0)[col_idx]))
                    }
                },
                None => ()
            };
            // Create empty projected table
            let mut table_project = Table::new();
            // Setup table
            match fields {
                Some(v) => {
                    for field in v {
                        handle!(match handle!(table.get_column(field)) {
                            Column::Boolean(_) => table_project.add_column(field, ColType::Boolean, CompressType::Uncompressed),
                            Column::Number(_) => table_project.add_column(field, ColType::Number, CompressType::Uncompressed),
                            Column::String(_) => table_project.add_column(field, ColType::String, CompressType::Uncompressed)
                        })
                    }
                },
                None => {
                    for field in table.get_headers() {
                        handle!(match handle!(table.get_column(field)) {
                            Column::Boolean(_) => table_project.add_column(field, ColType::Boolean, CompressType::Uncompressed),
                            Column::Number(_) => table_project.add_column(field, ColType::Number, CompressType::Uncompressed),
                            Column::String(_) => table_project.add_column(field, ColType::String, CompressType::Uncompressed)
                        })
                    }
                }
            };
            // Get limit of rows to add to table
            let lim_usize = match limit {
                Some(expr) => {
                    // Evaluate expression
                    let mut env = self.default_environment();
                    // Add aggregates into environment
                    table.push_aggregates(&mut env);
                    Some(handle!(eval_num(expr, &mut env)) as usize)
                },
                None => None
            };
            match sort_by {
                Some(_) => {
                    // Add rows to new table
                    for i in 0..full_rows.len() {
                        // Stop adding if reached limit
                        match lim_usize {
                            Some(x) => if i >= x {break},
                            None => ()
                        };
                        // Matching index of added row
                        let matching_index = full_rows[i].1;
                        // Add row to table
                        handle!(table_project.add_row(added_rows[matching_index].clone()))
                    };
                },
                None => {
                    // Reverse added rows (need to do this since popping)
                    added_rows.reverse();
                    // Add rows to new table
                    for i in 0..added_rows.len() {
                        // Stop adding if reached limit
                        match lim_usize {
                            Some(x) => if i >= x {break},
                            None => ()
                        };
                        // Add row to table
                        handle!(table_project.add_row(added_rows.pop().unwrap()))
                    };
                }
            }
            // Return new table
            QueryResult::Table(table_project)
        }
        fn create_const(&mut self, name: &String, expr: &Expr) -> QueryResult {
            // Evaluate expr
            let mut env = self.default_environment();
            let val = handle!(eval(expr, &mut env));
            // Check if name already in constants
            let pos = self.constants.iter().position(|r| r.0 == *name);
            match pos {
                Some(idx) => self.constants[idx] = (name.clone(), val),
                None => self.constants.push((name.clone(), val))
            };
            // Return nothing
            QueryResult::Success("Const ".to_string() + name)
        }
        fn create_column(&mut self, t: &ColType, s: &Option<CompressType>, col_name: &String, expr: &Expr, table_name: &String) -> QueryResult {
            // Get index of table
            let table_idx = handle!(self.get_table_index(table_name));
            let table = &self.tables[table_idx];
            // Calculate values for existing rows
            let col = match t {
                ColType::Boolean => {
                    // Data object
                    let mut col_data: Uncompressed<bool> = Uncompressed::new();
                    // Iterate through table rows
                    for row in table.iter() {
                        // Environment
                        let mut env = self.default_environment();
                        // Push row to environment
                        for field in table.get_headers() {
                            let idx = handle!(table.header_idx(field));
                            env.push(field, &row[idx]);
                        }
                        // Get column value
                        let val = handle!(eval_bool_option(expr, &mut env));
                        // Push value to data container
                        col_data.insert(val);
                    }
                    // Return column
                    Column::Boolean(Box::new(col_data))
                },
                ColType::Number => {
                    // Data object
                    let mut col_data: Uncompressed<f64> = Uncompressed::new();
                    // Iterate through table rows
                    for row in table.iter() {
                        // Environment
                        let mut env = self.default_environment();
                        // Push row to environment
                        for field in table.get_headers() {
                            let idx = handle!(table.header_idx(field));
                            env.push(field, &row[idx]);
                        }
                        // Get column value
                        let val = handle!(eval_num_option(expr, &mut env));
                        // Push value to data container
                        col_data.insert(val);
                    }
                    // Return column
                    Column::Number(Box::new(col_data))
                },
                ColType::String => {
                    // Data object
                    let mut col_data: Uncompressed<String> = Uncompressed::new();
                    // Iterate through table rows
                    for row in table.iter() {
                        // Environment
                        let mut env = self.default_environment();
                        // Push row to environment
                        for field in table.get_headers() {
                            let idx = handle!(table.header_idx(field));
                            env.push(field, &row[idx]);
                        }
                        // Get column value
                        let val = handle!(eval_str_option(expr, &mut env));
                        // Push value to data container
                        col_data.insert(val);
                    }
                    // Return column
                    Column::String(Box::new(col_data))
                }
            };
            // Get compression type
            let ctype = match s {
                Some(x) => *x,
                _ => CompressType::Uncompressed
            };
            // Borrow table as mutable
            let table = &mut self.tables[table_idx];
            // Insert column into table
            handle!(table.add_populated_column(col_name, col, ctype));
            // Mark column as calculated
            self.calculated[table_idx].push(Some(expr.clone()));
            // Return nothing
            QueryResult::Success("Column ".to_string() + col_name + " on " + table_name)
        }
        fn create_aggregate(&mut self, ag_name: &String, expr: &Expr, init: &Option<Expr>, table_name: &String) -> QueryResult {
            // Get index of table
            let table_idx = handle!(self.get_table_index(table_name));
            let table = &self.tables[table_idx];
            // Value of aggregate
            let mut ag_val = Val::NullVal;
            // Does this have an init?
            let has_init = match init {
                Some(_) => true,
                None => false
            };
            // Calculate aggregate for existing rows
            let mut i: usize = 0;
            for row in table.iter() {
                // Is this the first row?
                let first_row = i == 0;
                // Environment
                let mut env = self.default_environment();
                // Add row to environment
                for i in 0..row.len() {
                    env.push(&table.get_headers()[i], &row[i]);
                }
                // Add current value to environment, unless has init and is first row
                if !(has_init && first_row) {
                    env.push(&"current".to_string(), &ag_val);
                }
                // Evaluate
                ag_val = handle!(match first_row {
                    true => match init {
                        Some(e1) => eval(e1, &mut env),
                        None => eval(expr, &mut env)
                    },
                    false => eval(expr, &mut env)
                });
                // Increment i
                i += 1;
            };
            // Register aggregate into table
            let table = &mut self.tables[table_idx];
            table.add_aggregate(ag_name, &ag_val, expr, init);
            // Finished
            QueryResult::Success("Aggregate ".to_string() + ag_name + " on " + table_name)
        }
        fn select_aggregate(&mut self, ag_name: &String, table_name: &String) -> QueryResult {
            // Get index of table
            let table_idx = handle!(self.get_table_index(table_name));
            let table = &self.tables[table_idx];
            // Return aggregate
            QueryResult::Value(handle!(table.get_aggregate(ag_name)))
        }
        fn create_computation(&mut self, cmp_name: &String, expr: &Expr, table_name: &String) -> QueryResult {
            // Get index of table
            let table_idx = handle!(self.get_table_index(table_name));
            let table = &self.tables[table_idx]; 
            // Get value of computation (null if table is empty)
            let comp_val = match table.len() == 0 {
                true => Val::NullVal,
                false => {
                    // Environment
                    let mut env = self.default_environment();
                    table.push_aggregates(&mut env);
                    // Evaluate
                    handle!(eval(expr, &mut env))
                }
            };
            // Register computation into table
            let table = &mut self.tables[table_idx];
            table.add_computation(cmp_name, &comp_val, expr);
            // Return nothing
            QueryResult::Success("Computation ".to_string() + cmp_name + " on " + table_name)
        }
        fn select_computation(&mut self, cmp_name: &String, table_name: &String) -> QueryResult {
            // Get index of table
            let table_idx = handle!(self.get_table_index(table_name));
            let table = &self.tables[table_idx];
            // Return aggregate
            QueryResult::Value(handle!(table.get_computation(cmp_name)))
        }
        fn compress(&mut self, table_name: &String, fields: &Vec<String>, strats: &Vec<CompressType>) -> QueryResult {
            // Get index of table
            let table_idx = handle!(self.get_table_index(table_name));
            let table = &mut self.tables[table_idx];
            // Check fields and strats len
            if fields.len() != strats.len() { panic!("Unequal amount of fields and strategies") }
            // Call recompress on each column
            for i in 0..fields.len() {
                let col_idx = table.get_headers().iter().position(|r| *r == fields[i]).unwrap();
                handle!(table.recompress(col_idx, strats[i]))
            }
            // Return aggregate
            QueryResult::Success("Compression success on ".to_string() + table_name)
        }
        fn script(&mut self, expr: &Expr, tname: &Option<String>) -> QueryResult {
            // Setup environment
            let mut env = self.default_environment();
            match tname {
                Some(table_name) => {
                    // Get index of table
                    let table_idx = handle!(self.get_table_index(table_name));
                    let table = &mut self.tables[table_idx];
                    // Load table into env
                    table.push_all(&mut env);
                },
                _ => ()
            };
            // Evaluate expr
            let val = handle!(eval(expr, &mut env));
            // Return value
            QueryResult::Value(val)
        }
        fn import_csv(&mut self, cname: &String, tname: &String) -> QueryResult {
            // Get index of table
            let table_idx = handle!(self.get_table_index(tname));
            let table = &mut self.tables[table_idx];
            // Open CSV file
            let file = match File::open(cname) {
                Ok(f) => f,
                _ => return QueryResult::Error("Could not open file ".to_string() + cname)
            };
            // CSV reader
            let mut rdr = csv::Reader::from_reader(file);
            // First row should be headers
            let headers = match rdr.headers() {
                Ok(h) => h,
                Err(_) => return QueryResult::Error("Error reading CSV headers".to_string())
            };
            // Column types of each table column
            let col_types = table.get_col_types();
            // Map table headers to CSV headersd
            let mut h_map = Vec::new();
            let mut i = 0;
            for table_header in table.get_headers() {
                // Does table header exist in CSV headers?
                match headers.iter().position(|r| r.trim() == table_header) {
                    // If so, push mapping of table header idx -> column index
                    Some(idx) => h_map.push(Some((col_types[i], idx))),
                    // Otherwise, push none
                    None => {println!("{}", table_header); h_map.push(None)}
                }
                // Increment I
                i += 1;
            };
            // Insert rows
            for row in rdr.records() {
                match row {
                    // Row success
                    Ok(rec) => {
                        // Add data for each table row
                        let mut vals_insert = Vec::new();
                        for h in &h_map {
                            match h {
                                Some((ctype, row_idx)) => {
                                    // Get string of value to insert
                                    let val_str = &rec[*row_idx];
                                    // If value is empty, insert null
                                    if val_str == "" {
                                        vals_insert.push(Val::NullVal)
                                    }
                                    else {
                                        // Convert value and add according to column type
                                        vals_insert.push(match ctype {
                                            ColType::Boolean => Val::BoolVal(if val_str == "true" { true } else if val_str == "false" {false} else { return QueryResult::Error("Unexpeced boolean value ".to_string() + val_str + ". Expect either true or false") } ),
                                            ColType::Number => Val::NumVal(match val_str.parse() { Ok(f) => f, Err(_) => return QueryResult::Error("Error parsing float value ".to_string() + val_str) }),
                                            ColType::String => Val::StrVal(val_str.to_string())
                                        })
                                    }
                                },
                                None => vals_insert.push(Val::NullVal)
                            }
                        };
                        // Insert values into table
                        self.insert_and_update(tname, table_idx, vals_insert);
                    },
                    // Row not successful
                    Err(_) => continue
                }
            };
            QueryResult::Success("Import ".to_string() + cname)
        }
        pub fn execute(&mut self, q: String) -> QueryResult {
            // Parse given query
            let mut query_parser = Parser::new(q);
            let parsed_query = match query_parser.parse() {
                Ok(q) => q,
                Err(s) => return QueryResult::Error(s)
            };
            // Execute query
            match &parsed_query {
                Query::CreateTable(table_name, schema) => self.create_table(table_name, schema),
                Query::Insert(table_name, fields, values) => self.insert(table_name, fields, values),
                Query::Select(fields, table_name, where_, sort_by, limit) => self.select(fields, table_name, where_, sort_by, limit),
                Query::Const(name, expr) => self.create_const(name, expr),
                Query::Column(t, s, col_name, expr, table_name) => self.create_column(t, s, col_name, expr, table_name),
                Query::Aggregate(ag_name, expr, init, table_name) => self.create_aggregate(ag_name, expr, init, table_name),
                Query::SelectAggregate(ag_name, table_name) => self.select_aggregate(ag_name, table_name),
                Query::Comp(cmp_name, expr, table_name) => self.create_computation(cmp_name, expr, table_name),
                Query::SelectComp(cmp_name, table_name) => self.select_computation(cmp_name, table_name),
                Query::Compress(table_name, fields, strats) => self.compress(table_name, fields, strats),
                Query::Script(expr, tname) => self.script(expr, tname),
                Query::Exit => QueryResult::Exit,
                Query::ImportCSV(cname, tname) => self.import_csv(cname, tname),
                _ => panic!("Unimplemented")
            }
        }
        pub fn new() -> Database {
            Database {
                tables: Vec::new(),
                table_names: Vec::new(),
                constants: Vec::new(),
                calculated: Vec::new()
            }
        }
        pub fn get_table_index(&self, name: &String) -> Result<usize, String> { match self.table_names.iter().position(|r| *r == *name) { Some(i) => Ok(i), None => Err("Table ".to_string() + name + " does not exist")  } }
        pub fn get_table_names(&self) -> &Vec<String> { &self.table_names }
        pub fn default_environment(&self) -> Environment {  
            // New environment
            let mut def_env = Environment::new();
            // Add constants
            for c in self.constants.iter() {
                def_env.push(&c.0, &c.1)
            };
            // Return
            def_env
        }
    }
}