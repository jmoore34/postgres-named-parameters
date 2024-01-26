use postgresql_named_parameters::Query;

#[test]
fn macro_expansion_tests() {
    macrotest::expand("tests/expand/*.rs")
}

#[test]
fn basic() {
    #[derive(Query)]
    #[sql = "SELECT * FROM Person WHERE age = @age AND name = @name"]
    struct GetPeople {
        name: String,
        age: i32,
    }

    let _query = GetPeople {
        name: "Bob".into(),
        age: 18,
    };
    assert_eq!(
        GetPeople::sql(),
        "SELECT * FROM Person WHERE age = $2 AND name = $1"
    );
}

#[test]
fn escape() {
    #[derive(Query)]
    #[sql = "SELECT * FROM Person WHERE email LIKE '%@@gmail.com'"]
    struct GetPeople {}

    let _query = GetPeople {};
    assert_eq!(
        GetPeople::sql(),
        "SELECT * FROM Person WHERE email LIKE '%@gmail.com'"
    );
}

#[test]
fn repeated_arg() {
    #[derive(Query)]
    #[sql = "SELECT * FROM Employee WHERE first_name = @name OR last_name = @name"]
    struct GetPeople {
        name: String,
    }

    let _query = GetPeople { name: "Bob".into() };
    assert_eq!(
        GetPeople::sql(),
        "SELECT * FROM Employee WHERE first_name = $1 OR last_name = $1"
    );
}

#[test]
fn unit_struct() {
    #[derive(Query)]
    #[sql = "CREATE TABLE Users ()"]
    struct GetPeople;

    let _query = GetPeople {};
    assert_eq!(GetPeople::sql(), "CREATE TABLE Users ()");
}
