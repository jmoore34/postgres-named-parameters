// Export a wrapper around postgres_from_row::FromRow so we can reference it in
// our #[derive(Query)] procedural macro. We do not re-export the
// postgres_from_row crate because its #[derive(FromRow)] procedural macro does
// not work properly unless the user directly adds postgres_from_row to their
// Cargo.toml dependencies.
pub mod wrapper_for_derive_macro {
    use postgres_from_row::FromRow;
    pub fn try_from_row<T: FromRow>(row: &postgres::Row) -> Result<T, tokio_postgres::Error> {
        T::try_from_row(row)
    }
}
