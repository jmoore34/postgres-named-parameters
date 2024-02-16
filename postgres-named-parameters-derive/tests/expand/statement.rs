#[macro_use]
extern crate postgres_named_parameters_derive;
use postgres_named_parameters::*;

#[derive(Statement)]
#[statement(sql = "INSERT INTO Person VALUES @people")]
struct InsertPeople {
    people: Vec<Person>,
}
