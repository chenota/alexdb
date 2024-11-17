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