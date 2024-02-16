use postgres_from_row::FromRow;
use postgres_named_parameters::{Query, Statement};

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
    sql = "
        SELECT *
        FROM Person
        WHERE (first_name = @name OR last_name = @name)
        AND alive = @alive;",
    row = Person
)]
struct GetPeople<'a> {
    alive: bool,
    name: &'a str,
}

// Statements are like Queries except they do not return rows
// but rather an integer counting the number of rows affected.
// Hence, there is no `row` parameter to the `statement` attribute
// (unlike in the `query` attribute).
#[derive(Statement)]
#[statement(sql = "
CREATE TABLE IF NOT EXISTS Person (
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  hobby TEXT,
  alive BOOLEAN NOT NULL
)")]
struct CreatePersonTable;

#[derive(Statement)]
#[statement(sql = "DELETE FROM Person")]
struct TruncatePersonTable;

#[derive(Statement)]
#[statement(sql = "DELETE FROM Person WHERE id = @id")]
struct DeletePerson {
    id: i32
}

fn bulk_insert_people(
    db: &mut impl postgres::GenericClient,
    people: Vec<Person>,
) -> Result<u64, postgres::Error> {
    // There are various approaches for bulk queries like this one. Many ORMs
    // take the approach of constructing SQL at runtime. This library is
    // centered around SQL that is finalized at compile time, so we use a
    // different approach: we can split a vector of structs (Vec<Person>) into
    // one vector per column, and then reassemble them in Postgres.
    #[derive(Statement)]
    #[statement(sql = "
        INSERT INTO Person (first_name, last_name, hobby, alive)
        SELECT *
        FROM UNNEST(@first_names::TEXT[], @last_names::TEXT[], @hobbies::TEXT[], @alive_statuses::BOOL[])
    ")]
    struct InsertPeople {
        first_names: Vec<String>,
        last_names: Vec<String>,
        hobbies: Vec<Option<String>>,
        alive_statuses: Vec<bool>,
    }

    InsertPeople {
        first_names: people.iter().map(|p| p.first_name.clone()).collect(),
        last_names: people.iter().map(|p| p.last_name.clone()).collect(),
        hobbies: people.iter().map(|p| p.hobby.clone()).collect(),
        alive_statuses: people.iter().map(|p| p.alive).collect(),
    }
    .execute_statement(db)
}

fn main() -> Result<(), postgres::Error> {
    let connection_string = std::env::var("POSTGRES_CONNECTION_STRING")
        .unwrap_or("host=localhost user=postgres".to_owned());
    let mut db = postgres::Client::connect(&connection_string, postgres::NoTls)?;
    DeletePerson {
        id: 123
    }.execute_statement(&mut db)?;
    CreatePersonTable {}.execute_statement(&mut db)?;
    TruncatePersonTable {}.execute_statement(&mut db)?;



    let people_to_insert = vec![
        Person {
            first_name: "John".into(),
            last_name: "Doe".into(),
            hobby: None,
            alive: true,
        },
        Person {
            first_name: "Long".into(),
            last_name: "Da".into(),
            hobby: Some("Cello".into()),
            alive: true,
        },
    ];

    bulk_insert_people(&mut db, people_to_insert)?;

    let people = GetPeople {
        alive: true,
        name: "John",
    }
    .query_all(&mut db)?;
    // This roughly desugars to:
    // let people: Vec<Person> = db.query(
    //     "SELECT * FROM Person WHERE (first_name = $2 OR last_name = $2) AND alive = $1",
    //     &[&true, &"John"],
    // )?.iter().map(Person::try_from_row).collect::<Result<Vec<Person>,postgres::Error>>()?;

    println!("Found: {:?}", people);

    Ok(())
}
