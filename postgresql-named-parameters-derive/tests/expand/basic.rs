#[macro_use]
extern crate postgresql_named_parameters_derive;

#[derive(Query)]
#[sql = "SELECT * FROM Person WHERE age = @age AND name = @name"]
struct GetPeople {
    name: String,
    age: i32,
}
