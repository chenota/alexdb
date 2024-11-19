<div align="center">
  <picture>
    <source media="(prefers-color-scheme: light)" srcset="media/logo_horizontal.svg">
    <source media="(prefers-color-scheme: dark)" srcset="media/logo_horizontal_dark.svg">
    <img alt="AlexDB logo" src="media/logo_horizontal.svg" height="125">
  </picture>
</div>

##  AlexDB

AlexDB is an aggregation-focused relational database system for statistical analysis. It provides a dialect of SQL called SQLScript that embeds a functional scripting language in SQL queries.

## Using AlexDB: Standalone

AlexDB provides a REPL that acts as its main entry point; `main.rs` loads the REPL, thus it's started by simply executing the main program.

## Using AlexDB: Library

If you wish to use AlexDB as a library, the database object (found in `engine/database.rs`) is the main entry point into AlexDB; a single instance of `Database` is a single instance of AlexDB. You can create a new database by using `Database::new()` and execute queries on the database by calling `database.execute(query: str)`, which returns a `QueryResult`. Some other useful constructs to be aware of are the types found under `sqlscript/types` (specifically `Val` which holds the SQLScript types) and the `Table` object found under `storage/table`.

## SQLScript

SQLScript is a novel variant of SQL that embeds a scripting language directly in queries, which allows queries to be very complex while not needing to build much complexity into the system itself. There's a lot of moving parts here, so let's break it down.

### Script

The *Script* part of SQLScript acts as a standalone programming language, which you can directly try out by using the `SCRIPT <script>` query. This language was heavily inspired by JavaScript (in fact, it could be trivially converted thereof), and is functional and purely immutable. 

#### Values

Every expression reduces to a value, some of which can be stored in the database and some of which cannot. Here are the kinds of values that SQLScript provides:

| Value       | Can be Stored |
|-------------|:-------------:|
| `num (f64)` | ✅           |
| `bool`      | ✅           |
| `str`       | ✅           |
| `null`      | ✅           |
| `undefined` | ❌           |
| `tuple`     | ❌           |
| `closure`   | ❌           |

#### Unary Operators

SQLScript provides a wide variety of unary operators. A unary operator operates on a singular value.

| Operator | Description                           |
|----------|---------------------------------------|
| `-x`     | Negates `x`. Always returns a `num`.  |
| `!x`     | Toggles `x`. Always returns a `bool`. |
| `?x`     | Converts x to a `bool`.               |
| `&x`     | Converts x to a `str`.                |
| `+x`     | Converts x to a `num`.                |
| `^x`     | Performs `ceil(x)`                    |
| `_x`     | Performs `floor(x)`                   |
| `x()`    | Calls `x`                             |

#### Binary Operators

SQLScript provides a wide variety of binary operators. A binary operator operates on two values. All operators listed in order of precedence (lowest to highest).

| Operator   | Description                                                                           |
|------------|---------------------------------------------------------------------------------------|
| `x && y`   | Performs a JS-style logical AND                                                       |
| `x \|\| y` | Performs a JS-style logical OR                                                        |
|            |                                                                                       |
| `x > y`    | Strictly greater than. Performs string comparison if both `x` and `y` are strings.    |
| `x >= y`   | Greater than or equal to. Performs string comparison if both `x` and `y` are strings. |
| `x < y`    | Strictly less than. Performs string comparison if both `x` and `y` are strings.       |
| `x <= y`   | Less than or equal to. Performs string comparison if both `x` and `y` are strings.    |
| `x == y`   | Loose equality                                                                        |
| `x === y`  | Strict equality                                                                       |
|            |                                                                                       |
| `x + y`    | Addition. Performs string concatenation if both `x` and `y` are strings.              |
| `x - y`    | Subtraction                                                                           |
|            |                                                                                       |
| `x * y`    | Multiplication                                                                        |
| `x / y`    | Division                                                                              |
| `x % y`    | Modulo                                                                                |
|            |                                                                                       |
| `x.y`      | Tuple access                                                                          |

#### Conditionals

Conditional statement always return a value.

`if x then y else z`

#### Blocks

SQLScript implements blocks that have their own scope and allow you to store values in variables. Blocks evaluate to values, thus the last item in a block has to be a bare expression. Everything is immutable, therefore no need for `let` or `const` keywords!

`{x = ...; y = ...; ...}`

#### Function Closures

SQLScript functions implement lexical scope via closures that capture the state of the program when the function was defined. Variables will default to their define-time value and only fall back on the call-time environment if they didn't exist when the function was defined.

`fun -> ...`

`fun x, y, ... -> ...`

#### Tuples

Tuples are fixed-length and hold multiple values; they are zero-indexed and accessed by the dot operator.

`[x, y, ...]`

`tup.i`

### SQL

The *SQL* part of SQLScript is how you interact with the database. Let's go over all the available queries.

#### Create Table

Create table is how you define schemas for new tables. This query acts very similarly to its SQL counterpart.

Syntax: `CREATE TABLE name (field type [compression], field type [compression], ...)`

If you don't defined a compression scheme in which to compress the column, SQLScript will default to `none`.

Example: `CREATE TABLE person (name str, age num xor, height num xor, has_degree bool)`

#### Insert Values

Insert values is used to insert individual values into the table; this query works much like its SQL counterpart, with the exception that inserted values can be an arbitrary SQLScript expression. If you choose to specify fields and omit any, then `null` will be inserted into omitted fields.

Syntax: `INSERT INTO table [(field, field, ...)] VALUES (expr, expr, ...)`

Example: `INSERT INTO person (name, age, height) VALUES ('Earl', 87, 5.25)`

| Variable Scope     | Has Access |
|--------------------|:----------:|
| Global Constants   | ✅        |
| Table Rows         | ❌        |
| Table Aggregates   | ❌        |
| Table Computations | ❌        |

#### Create Const

Create const allows you to define a global database-wide constant.

Syntax: `CREATE CONST name = expr`

Example: `CREATE CONST max = fun x, y -> if x > y then x else y`

| Variable Scope     | Has Access |
|--------------------|:----------:|
| Global Constants   | ❌        |
| Table Rows         | ❌        |
| Table Aggregates   | ❌        |
| Table Computations | ❌        |

#### Create Column

Create column allows you to add calculated columns into a table schema using the script language.

Syntax: `CREATE COLUMN (type [compression]) name = expr INTO table`

Example: `CREATE COLUMN (bool) is_adult = age >= 18 INTO person`

| Variable Scope     | Has Access |
|--------------------|:----------:|
| Global Constants   | ✅        |
| Table Rows         | ✅        |
| Table Aggregates   | ❌        |
| Table Computations | ❌        |

#### Create Aggregate

Create aggregate allows you to define an aggregate on a table. Aggregates are calculated via the fold/reduce paradigm, thus aggregate calculators only have access to the most recent inserted row and the current aggregate value, stored in `current`. If defined, the `INIT` expression will use the first inserted value to initialize the aggregate, otherwise the aggregate is initialized with `null`. Unlike calculated columns, aggregates and computations can be any SQLScript value, like tuples!

Syntax: `CREATE AGGREGATE name = expr [INIT expr] INTO table`

Example: `CREATE AGGREGATE max_age = max(age, current) INIT age INTO person` where `max` is a globally-defined constant.

| Variable Scope     | Has Access |
|--------------------|:----------:|
| Global Constants   | ✅        |
| Table Rows         | ✅        |
| Table Aggregates   | ❌        |
| Table Computations | ❌        |

#### Create Computation

Create aggregate allows you to define a computation on a table. Computations are used to combine table aggregates since aggregates are isolated from each other.

Syntax: `CREATE COMP name = expr INTO table`

Example: `CREATE COMP avg_age = sum_age / num_rows INTO person` where `sum_age` and `num_rows` are aggregates.

| Variable Scope     | Has Access |
|--------------------|:----------:|
| Global Constants   | ✅        |
| Table Rows         | ✅        |
| Table Aggregates   | ✅        |
| Table Computations | ❌        |

#### Script

The script query lets you perform any arbitrary one-time code execution. You may optionally provide your expression with values from a table.

Syntax: `SCRIPT expr [FROM table]`

Example: `SCRIPT max_age / 2 FROM person` where `max_age` is an aggregate.

| Variable Scope     | Has Access |
|--------------------|:----------:|
| Global Constants   | ✅        |
| Table Rows         | ❌        |
| Table Aggregates   | ✅        |
| Table Computations | ✅        |

#### Select

The generic select query allows you to select rows from a table and provides much of the functionality of traditional SQL select queries. Notice that the `WHERE` clause evaluates an arbitrary expression and that `SELECT` has access to table aggregates and constants, which opens up some very interesting and complex query opportunities.

Syntax: `SELECT * | field, field, ... FROM table [WHERE expr] [ORDER BY field [ASC | DESC]] [LIMIT expr] [EXPORT CSV 'path/to/csv.csv']`

Example: `SELECT name FROM person WHERE age > avg_age ORDER BY height DESC LIMIT 5` where `avg_age` is an aggregate.

| Variable Scope     | Has Access |
|--------------------|:----------:|
| Global Constants   | ✅        |
| Table Rows         | ✅        |
| Table Aggregates   | ✅        |
| Table Computations | ✅        |

#### Select Aggregate

This query allows you to view an aggregate from a table.

Syntax: `SELECT AGGREGATE name FROM table`

#### Select Computation

This query allows you to view a computation from a table.

Syntax: `SELECT COMP name FROM table`

#### Import CSV

The import CSV query lets you read data from a CSV file into an existing database schema.

Syntax: `IMPORT CSV 'path/to/csv.csv' INTO table`

Example: `IMPORT CSV 'people.csv' INTO person`

#### Export CSV

The export CSV query lets you save data from a table into a CSV file.

Syntax: `EXPORT CSV 'path/to/csv.csv' FROM table`

Example: `EXPORT CSV 'people.csv' FROM person`

#### Compress

The compress query allows you to change the type of compression used on each column. When specifying strategies, you can either specify a single strategy per indicated field or a single strategy for all indicated fields. Any existing data will be re-compressed using the indicated strategy.

Syntax: `COMPRESS table (field, field, ...) (strategy, strategy, ...) | strategy`

Example: `COMPRESS person (age, height) xor`

## Data Compression

AlexDB compresses columns of data, and the user can specify between `{runlen, bitmap, xor, none}`. 

### Run Length

Run length compression squeezes together contiguous identical values. This compression scheme works best when you have few distinct values that are grouped together.

Available for: `str`, `num`

Example: $[4, 4, 4, 5, 5] \to [(4, 3), (5, 2)]$

### Bitmap

Bitmap encoding assigns a bitmap to each unique value to indicate which rows it appears in. This compression scheme works best when you have few distinct values that are scattered.

Available for: `str`, `num`

Example: $[4, 5, 4, 5, 4] \to [(4, 10101), (5, 01010)]$

### Xor

Xor encoding is a type of delta-encoding that XORs consecutive values and stores only the meaningful bits of the XOR. The idea is that if consecutive values don't differ much, then their XOR will have mostly zeros which means less bits to store. This compression scheme works best on data where consecutive values don't differ much.

Available for: `num`

### Booleans

You may have noticed that `bool`s have been left out of every stated compression scheme, which is because `bool`s can be stored very efficiently in a bit vector and the only strategy that could *possibly* improve compression, `runlen`, would only do so under very specific and unlikely circumstances.

## Examples

- [Vector Clock Messaging System](media/vc_example.md)

## Future Work

AlexDB could be reasonably extended in the following ways:
- Nested queries
- Table joins
- More compression schemes
- More ways of interacting with AlexDB, like websockets or Python bindings

## Thoughts and Acknowledgements

AlexDB was one of my many semester projects for the Fall 2024 semester. This was by far my favorite project and the one I spent the most time working on, and I hope that my passion for programming languages, databases, and computer science really shines through! I'm excited to take what I've learned while developing AlexDB and apply it to many exciting projects in the future.