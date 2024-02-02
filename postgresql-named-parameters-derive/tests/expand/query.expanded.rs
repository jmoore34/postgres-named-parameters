#[macro_use]
extern crate postgresql_named_parameters_derive;
extern crate postgres_from_row;
use postgresql_named_parameters::*;
use postgres_from_row::FromRow;
struct Person {
    first_name: String,
    last_name: String,
    age: i32,
    alive: bool,
}
#[query(
    sql = "SELECT * FROM Person WHERE (first_name = @name OR last_name = @name) AND alive = @alive",
    row = Person
)]
struct GetPeople<'a> {
    alive: bool,
    name: &'a str,
}
#[automatically_derived]
impl<'a> postgresql_named_parameters::Query for GetPeople<'a> {
    type Row = Person;
    fn query_all(
        &self,
        connection: &mut impl postgresql_named_parameters::postgres::GenericClient,
    ) -> Result<Vec<Self::Row>, postgresql_named_parameters::postgres::error::Error> {
        let rows = connection
            .query(
                "SELECT * FROM Person WHERE (first_name = $2 OR last_name = $2) AND alive = $1",
                &[&self.alive, &self.name],
            )?;
        rows.iter()
            .map(
                postgresql_named_parameters::internal::wrapper_for_derive_macro::try_from_row::<
                    Self::Row,
                >,
            )
            .collect()
    }
    fn query_opt(
        &self,
        connection: &mut impl postgresql_named_parameters::postgres::GenericClient,
    ) -> Result<Option<Self::Row>, postgresql_named_parameters::postgres::error::Error> {
        let maybe_row = connection
            .query_opt(
                "SELECT * FROM Person WHERE (first_name = $2 OR last_name = $2) AND alive = $1",
                &[&self.alive, &self.name],
            )?;
        match maybe_row {
            None => Ok(None),
            Some(row) => {
                let decoded_row = postgresql_named_parameters::internal::wrapper_for_derive_macro::try_from_row::<
                    Self::Row,
                >(&row)?;
                Ok(Some(decoded_row))
            }
        }
    }
    fn query_one(
        &self,
        connection: &mut impl postgresql_named_parameters::postgres::GenericClient,
    ) -> Result<Self::Row, postgresql_named_parameters::postgres::error::Error> {
        let row = connection
            .query_one(
                "SELECT * FROM Person WHERE (first_name = $2 OR last_name = $2) AND alive = $1",
                &[&self.alive, &self.name],
            )?;
        postgresql_named_parameters::internal::wrapper_for_derive_macro::try_from_row::<
            Self::Row,
        >(&row)
    }
}
