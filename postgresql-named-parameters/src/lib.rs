pub use postgres;
pub use postgres_from_row;
pub use postgresql_named_parameters_derive::*;
pub use tokio_postgres;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub trait Query {
    fn sql() -> &'static str;
    fn parameter_names() -> &'static str;
    fn execute(&self, connection: &mut postgres::Client) -> Result<u64, postgres::error::Error>;
}

pub struct MyQuery {}

use postgres_from_row::FromRow;
pub trait QueryAll {
    type Row: postgres_from_row::FromRow;
    fn query_all(
        &self,
        connection: &mut postgres::Client,
    ) -> Result<Vec<Self::Row>, postgres::error::Error> {
        connection
            .query("query", &[])
            .and_then(|rows| rows.iter().map(Self::Row::try_from_row).collect())
    }
    fn query_all_in_transaction(
        &self,
        transaction: &mut postgres::Transaction  ,
    ) -> Result<Vec<Self::Row>, postgres::error::Error> {
        transaction
            .query("query", &[])
            .and_then(|rows| rows.iter().map(Self::Row::try_from_row).collect())
    }
}

#[cfg(test)]
mod tests {}
