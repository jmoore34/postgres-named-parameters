#[macro_use]
extern crate postgresql_named_parameters_derive;
use postgresql_named_parameters::*;
#[statement(sql = "INSERT INTO Person VALUES @people")]
struct InsertPeople {
    people: Vec<Person>,
}
#[automatically_derived]
impl postgresql_named_parameters::Statement for InsertPeople {
    fn execute_statement(
        &self,
        connection: &mut impl postgresql_named_parameters::postgres::GenericClient,
    ) -> Result<u64, postgresql_named_parameters::postgres::error::Error> {
        connection.execute("INSERT INTO Person VALUES $1", &[&self.people])
    }
}
