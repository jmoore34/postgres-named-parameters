# postgresql-named-parameters

`postgresql-named-parameters` is a lightweight wrapper around the `postgres`
crate which provides the ergonomics of named parameters using numbered
parameters under the hood.

## Usage example

```rust,no_run
use postgres_from_row::FromRow;
use postgresql_named_parameters::Query;

// Use the postgres-from-row crate to deserialize the rows returned
// from queries into a struct
#[derive(FromRow, Debug)]
struct Person {
    first_name: String,
    last_name: String,
    hobby: Option<String>,
    alive: bool,
}

// Use a struct to define a query. The #[derive(Query)] will implement
// query functions that handle passing in the parameters, and the named
// parameters are converted to numbered parameters ($1, $2,...) at compile
// time.
#[derive(Query)]
#[query(
    sql = "SELECT * FROM Person WHERE (first_name = @name OR last_name = @name) AND alive = @alive",
    row = Person
)]
struct GetPeople<'a> {
    alive: bool,
    name: &'a str,
}

fn main() -> Result<(), postgres::Error> {
    let connection_string = std::env::var("POSTGRES_CONNECTION_STRING")
        .unwrap_or("host=localhost user=postgres".to_owned());
    let mut db = postgres::Client::connect(&connection_string, postgres::NoTls)?;

    let people: Vec<Person> = GetPeople {
        alive: true,
        name: "John",
    }
    .query_all(&mut db)?;

    // This roughly desugars to:
    // let people: Vec<Person> = db.query(
    //     "SELECT * FROM Person WHERE (first_name = $2 OR last_name = $2) AND alive = $1",
    //     &[&true, &"John"],
    // )?.iter().map(Person::try_from_row).collect::<Result<Vec<Person>,postgres::Error>>()?;
    // Note that the #[derive(Query)] takes care of changing the SQL to use
    // numbered parameters (i.e. $1, $2) at compile time.

    println!("Found: {:?}", people);

    Ok(())
}
```
# Features

* Supports transactions
* SQL transformation to numbered parameters happens at compile time
* Mis-typing a named parameter (e.g. `@naame` instead of `@name`) produces a
  compiler error


# Attribution & Related Libraries

This crate was inspired by the following libraries:

* [couch/aykroyd](https://git.sr.ht/~couch/aykroyd) provided the idea of
  representing queries as structs
* [nolanderc/rust-postgres-query](https://github.com/nolanderc/rust-postgres-query)
  and
  [solidsnack/rust-postgres-named-parameters](https://github.com/solidsnack/rust-postgres-named-parameters)
  were used as reference when creating the SQL parser
* [3noch/postgresql-simple-interpolate](https://github.com/3noch/postgresql-simple-interpolate)
  is the Haskell library that motivated creating an ergonomic way of dealing
  with SQL parameters in Rust