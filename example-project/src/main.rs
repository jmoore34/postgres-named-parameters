use postgres_from_row::{tokio_postgres::types::ToSql, FromRow};
use postgresql_named_parameters::{Query, Statement};

#[derive(FromRow, ToSql, Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: i32,
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
#[statement(sql = "INSERT INTO Person VALUES @people")]
struct InsertPeople {
    people: Vec<Person>,
}

fn main() {
    let mut client =
        postgres::Client::connect("host=localhost user=postgres", postgres::NoTls).unwrap();

    let people_to_insert = vec![
        Person {
            first_name: "John".into(),
            last_name: "Doe".into(),
            age: 22,
            alive: true,
        },
        Person {
            first_name: "Long".into(),
            last_name: "Da".into(),
            age: 22,
            alive: true,
        },
    ];

    InsertPeople { people: people_to_insert }
        .execute(&mut client)
        .unwrap();

    let people = GetPeople {
        alive: true,
        name: "John".into()
    }.query_all(&mut client).unwrap();

    println!("Found: {:?}", people);
}
