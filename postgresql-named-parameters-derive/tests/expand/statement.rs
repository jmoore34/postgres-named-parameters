#[macro_use]
extern crate postgresql_named_parameters_derive;
use postgresql_named_parameters::*;

#[derive(Statement)]
#[statement(sql = "INSERT INTO Person VALUES @people")]
struct InsertPeople {
    people: Vec<Person>,
}