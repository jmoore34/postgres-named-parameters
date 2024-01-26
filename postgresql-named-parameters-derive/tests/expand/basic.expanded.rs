#[macro_use]
extern crate postgresql_named_parameters_derive;
#[sql = "SELECT * FROM Person WHERE age = @age AND name = @name"]
struct GetPeople {
    name: String,
    age: i32,
}
#[automatically_derived]
impl postgresql_named_parameters::Query for GetPeople {
    fn sql() -> &'static str {
        "SELECT * FROM Person WHERE age = $2 AND name = $1"
    }
    fn parameter_names() -> &'static str {
        "& [& self.name, & self.age]"
    }
    fn execute(
        &self,
        connection: &mut postgres::Client,
    ) -> Result<u64, postgres::error::Error> {
        connection
            .execute(
                "SELECT * FROM Person WHERE age = $2 AND name = $1",
                &[&self.name, &self.age],
            )
    }
}
