# postgres-named-parameters

`postgres-named-parameters` is a lightweight macro wrapper around the `postgres`
crate which gives you the ergonomics of named parameters in your raw SQL
queries. Under the hood, your named parameters are transformed into numbered
parameters at compile time.

## Usage example

```rust,no_run
use postgres_from_row::FromRow;
use postgres_named_parameters::Query;

// Use the postgres-from-row crate to decode each row returned
// from a query into a struct.
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
    // Write the query using named parameters
    sql = "
      SELECT *
      FROM Person
      WHERE (first_name = @name OR last_name = @name)
      AND alive = @alive",
    // Specify what type each row returned from the query should decode to
    row = Person
)]
struct GetPeople<'a> {
    // Define the query's parameters
    alive: bool,
    name: &'a str,
}

fn main() -> Result<(), postgres::Error> {
    let connection_string = std::env::var("POSTGRES_CONNECTION_STRING")
        .unwrap_or("host=localhost user=postgres".to_owned());
    let mut db = postgres::Client::connect(&connection_string, postgres::NoTls)?;

    // Execute the query
    let people: Vec<Person> = GetPeople {
        alive: true,
        name: "John",
    }
    .query_all(&mut db)?;

    // This roughly desugars to:
    //
    // let people: Vec<Person> = db.query(
    //     "SELECT *
    //      FROM Person
    //      WHERE (first_name = $2 OR last_name = $2)
    //      AND alive = $1",
    //     &[&true, &"John"],
    // )?
    // .iter()
    // .map(Person::try_from_row)
    // .collect::<Result<Vec<Person>,postgres::Error>>()?;
    //
    // Note that the #[derive(Query)] takes care of changing the SQL to use
    // numbered parameters (i.e. $1, $2) at compile time.

    println!("Found: {:?}", people);

    Ok(())
}
```
For a more thorough example (including bulk queries), see the example
project folder in the [GitHub repository](https://github.com/jmoore34/postgres-named-parameters).

# Features

* Supports transactions
* SQL transformation to numbered parameters happens at compile time
* Mis-typing a named parameter (e.g. `@naame` instead of `@name`) produces a
  compile-time error


# Attribution & Related Libraries

This crate was inspired by the following libraries:

* [couch/aykroyd](https://git.sr.ht/~couch/aykroyd) provided the idea of
  representing queries as structs
* [nolanderc/rust-postgres-query](https://github.com/nolanderc/rust-postgres-query)
  and
  [solidsnack/rust-postgres-named-parameters](https://github.com/solidsnack/rust-postgres-named-parameters)
  were used as reference when creating the SQL parser
* [3noch/postgres-simple-interpolate](https://github.com/3noch/postgresql-simple-interpolate)
  is the Haskell library that motivated creating an ergonomic way of dealing
  with SQL parameters in Rust