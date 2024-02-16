#[macro_use]
extern crate postgres_named_parameters_derive;
use postgres_named_parameters::*;
#[statement(sql = "INSERT INTO Person VALUES @people")]
struct InsertPeople {
    people: Vec<Person>,
}
#[automatically_derived]
impl postgres_named_parameters::Statement for InsertPeople {
    fn execute_statement(
        &self,
        connection: &mut impl postgres_named_parameters::postgres::GenericClient,
    ) -> Result<u64, postgres_named_parameters::postgres::error::Error> {
        connection.execute("INSERT INTO Person VALUES $1", &[&self.people])
    }
}
