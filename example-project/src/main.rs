use postgres_from_row::{tokio_postgres::types::ToSql, FromRow};
use postgresql_named_parameters::{Query, Statement};

#[derive(FromRow, ToSql, Debug)]
struct Person {
    first_name: String,
    last_name: String,
    hobby: Option<String>,
    alive: bool,
}

#[derive(Query)]
#[query(
    sql = "SELECT * FROM Person WHERE (first_name = @name OR last_name = @name) AND alive = @alive",
    row = Person
)]
struct GetPeople<'a> {
    alive: bool,
    name: &'a str,
}

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

fn main() {
    let connection_string = std::env::var("POSTGRES_CONNECTION_STRING")
        .unwrap_or("host=localhost user=postgres".to_owned());
    let mut db = postgres::Client::connect(&connection_string, postgres::NoTls).unwrap();

    CreatePersonTable {}.execute_statement(&mut db).unwrap();
    TruncatePersonTable {}.execute_statement(&mut db).unwrap();

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

    bulk_insert_people(&mut db, people_to_insert).unwrap();

    let people = GetPeople {
        alive: true,
        name: "John".into(),
    }
    .query_all(&mut db)
    .unwrap();

    println!("Found: {:?}", people);
}