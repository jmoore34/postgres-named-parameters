pub mod internal;

pub use postgres;
pub use postgresql_named_parameters_derive::*;

pub trait Statement {
    fn execute_statement(&self, connection: &mut postgres::Client) -> Result<u64, postgres::error::Error>;
}

pub trait Query {
    type Row: postgres_from_row::FromRow;
    fn query_all(
        &self,
        connection: &mut impl postgres::GenericClient,
    ) -> Result<Vec<Self::Row>, postgres::error::Error>;

    fn query_opt(
        &self,
        connection: &mut impl postgres::GenericClient,
    ) -> Result<Option<Self::Row>, postgres::error::Error>;

    fn query_one(
        &self,
        connection: &mut impl postgres::GenericClient,
    ) -> Result<Self::Row, postgres::error::Error>;
}