#![warn(missing_docs)]
#![doc = include_str!("../../README.md")]
pub mod internal;

pub use postgres;
/// See the [Query] docs for details.
pub use postgres_named_parameters_derive::Query;
/// See the [Statement] docs for details.
pub use postgres_named_parameters_derive::Statement;

/// A Statement is a SQL statement that, unlike a [Query], does not return rows.
/// Instead, it returns the number of rows that have been affected by the
/// statement.
///
/// # Example
/// `Statement` can be derived like so:
/// ```no_run
/// #[derive(Statement)]
/// #[statement(sql = "DELETE FROM Person WHERE id = @id")]
/// struct DeletePerson {
///     id: i32
/// }
/// ```
///
/// It then can be used like so:
/// ```no_run
/// # #[derive(Statement)]
/// # #[statement(sql = "DELETE FROM Person WHERE id = @id")]
/// # struct DeletePerson {
/// #     id: i32
/// # }
/// fn main() -> Result<(), postgres::Error> {
///     let connection_string = std::env::var("POSTGRES_CONNECTION_STRING")
///         .unwrap_or("host=localhost user=postgres".to_owned());
///     let mut db = postgres::Client::connect(&connection_string, postgres::NoTls)?;
///
///     let delete_count = DeletePerson {
///         id: 123
///     }.execute_statement(&mut db)?;
///
///     println!("Deleted {} people", delete_count);
///     Ok(())
/// }
/// ```
/// For a more thorough example (including bulk queries), see the example
/// project folder in the [GitHub repository](https://github.com/jmoore34/postgres-named-parameters).
///
/// # Notes
/// * In order to use `#[derive(Statement)`, you must also provide the helper
///   attribute `#[statement(sql = "...")`
///     * The `sql` parameter is required and must be a string literal
///     * Unlike [Query], there is no `row` parameter because
///       `Statement` does not return rows (but rather a count of the number of
///       rows affected)
/// * At compile time, the derive macro will check that the parameter names you
///   used in your query match the field names defined in the struct. A mismatch
///   (e.g. using "@idd" instead of "@id" when the field name is `id`) will
///   cause a compiler error.
/// * If you want to include a single literal `@` in your SQL, you must escape
///   it by doubling it (`@@`)
pub trait Statement {
    /// Execute a given statement on a given database connection or transaction,
    /// and return the number of rows that were affected.
    ///
    /// For the sole argument you can pass either a database connection (i.e.
    /// [postgres::Client]) or a transaction (i.e. [postgres::Transaction]).
    fn execute_statement(
        &self,
        connection: &mut impl postgres::GenericClient,
    ) -> Result<u64, postgres::error::Error>;
}

/// A Query is a SQL query that, unlike a [Statement], returns
/// rows from the database.
///
/// # Example
/// `Query` can be derived like so:
/// ```no_run
/// # #[derive(FromRow, Debug)]
/// # struct Person {
/// #     first_name: String,
/// #     last_name: String,
/// #     hobby: Option<String>,
/// #     alive: bool,
/// # }
/// #[derive(Query)]
/// #[query(
///     // Write the query using named parameters
///     sql = "
///       SELECT *
///       FROM Person
///       WHERE (first_name = @name OR last_name = @name)
///       AND alive = @alive",
///     // Specify what type the rows should be decoded to
///     row = Person
/// )]
/// struct GetPeople<'a> {
///     // Define the query parameters
///     alive: bool,
///     name: &'a str,
/// }
/// ```
/// Note that the struct you specify for `row` must implement
/// [FromRow](postgres_from_row::FromRow). You can do this by using
/// `#[derive(FromRow)`, which you can get by adding
/// [postgres-from-row](https://crates.io/crates/postgres-from-row) to your
/// `Cargo.toml`.
/// ```no_run
/// #[derive(FromRow, Debug)]
/// struct Person {
///     first_name: String,
///     last_name: String,
///     hobby: Option<String>,
///     alive: bool,
/// }
/// ````
///
/// Your can then use your query like this:
/// ```no_run
/// # #[derive(FromRow, Debug)]
/// # struct Person {
/// #     first_name: String,
/// #     last_name: String,
/// #     hobby: Option<String>,
/// #     alive: bool,
/// # }
/// # #[query(
/// #     // Write the query using named parameters
/// #     sql = "
/// #       SELECT *
/// #       FROM Person
/// #       WHERE (first_name = @name OR last_name = @name)
/// #       AND alive = @alive",
/// #     // Specify what type the rows should be decoded to
/// #     row = Person
/// # )]
/// # struct GetPeople<'a> {
/// #     alive: bool,
/// #     name: &'a str,
/// # }
/// fn main() -> Result<(), postgres::Error> {
///     let connection_string = std::env::var("POSTGRES_CONNECTION_STRING")
///         .unwrap_or("host=localhost user=postgres".to_owned());
///     let mut db = postgres::Client::connect(&connection_string, postgres::NoTls)?;
///
///     let people = GetPeople {
///         alive: true,
///         name: "John",
///     }
///     .query_all(&mut db)?;
///
///     println!("Found: {:?}", people);
///     Ok(())
/// }
/// ```
/// At compile time, the SQL is transformed to use numbered parameters. For
/// example, the above query will invoke:
/// ```no_run
/// # fn main() -> Result<(), postgres::Error> {
/// #     let connection_string = std::env::var("POSTGRES_CONNECTION_STRING")
/// #         .unwrap_or("host=localhost user=postgres".to_owned());
/// #     let mut db = postgres::Client::connect(&connection_string, postgres::NoTls)?;
/// let people: Vec<Person> = db.query(
///     "SELECT *
///      FROM Person
///      WHERE (first_name = $2 OR last_name = $2)
///      AND alive = $1",
///     &[&true, &"John"],
/// )?
/// .iter()
/// .map(Person::try_from_row)
/// .collect::<Result<Vec<Person>,postgres::Error>>()?;
/// #   Ok(())
/// # }
/// ````
///
/// For a more thorough example (including bulk queries), see the example
/// project folder in the [GitHub repository](https://github.com/jmoore34/postgres-named-parameters).
///
/// # Notes
/// * In order to use `#[derive(Query)`, you must also provide the helper
///   attribute `#[statement(sql = "...", row = SomeStruct)`
///     * Both the `sql` and `row` parameters are required
///     * The `sql` parameter must be a string literal
///     * The `row` parameter must implement
///       [FromRow](postgres_from_row::FromRow) (see above).
/// * At compile time, the derive macro will check that the parameter names you
///   used in your query match the field names defined in the struct. A mismatch
///   (e.g. using "@naame" instead of "@name" when the field name is `id`) will
///   cause a compiler error.
/// * If you want to include a single literal `@` in your SQL, you must escape
///   it by doubling it (`@@`)
pub trait Query {
    /// The type that each individual row returned from the query should decode
    /// to. Note that the struct you specify for `row` must implement
    /// [FromRow](postgres_from_row::FromRow). You can do this by using
    /// `#[derive(FromRow)`, which you can get by adding
    /// [postgres-from-row](https://crates.io/crates/postgres-from-row) to your
    /// `Cargo.toml`.
    type Row: postgres_from_row::FromRow;

    /// Run the query and return all the rows in a vector.
    ///
    /// For the sole argument you can pass either a database connection (i.e.
    /// [postgres::Client]) or a transaction (i.e. [postgres::Transaction]).
    fn query_all(
        &self,
        connection: &mut impl postgres::GenericClient,
    ) -> Result<Vec<Self::Row>, postgres::error::Error>;

    /// Run the query, expecting exactly one or zero rows. Return `None` if
    /// there are no rows, and return an error if there are more than one rows.
    /// See also: `query_one`.
    /// For the sole argument you can pass either a database connection (i.e.
    /// [postgres::Client]) or a transaction (i.e. [postgres::Transaction]).
    fn query_opt(
        &self,
        connection: &mut impl postgres::GenericClient,
    ) -> Result<Option<Self::Row>, postgres::error::Error>;

    /// Run the query, expecting exactly one row. Return an error if zero or
    /// more than one row are returned. See also: `query_opt`.
    ///
    /// For the sole argument you can pass either a database connection (i.e.
    /// [postgres::Client]) or a transaction (i.e. [postgres::Transaction]).
    fn query_one(
        &self,
        connection: &mut impl postgres::GenericClient,
    ) -> Result<Self::Row, postgres::error::Error>;
}
