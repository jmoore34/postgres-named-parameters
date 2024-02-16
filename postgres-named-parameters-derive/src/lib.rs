use syn::DeriveInput;

mod numberify;
mod query;
mod util;
mod statement;

/// Derives the [postgres-named-parameters::Query] trait. See the
/// [postgres-named-parameters::Query] docs for details.
#[proc_macro_derive(Query, attributes(query))]
pub fn derive_query(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    query::derive_query_impl(ast)
        .unwrap_or_else(|err| err.into_compile_error())
        .into()
}

/// Derives the [postgres-named-parameters::Statement] trait. See the
/// [postgres-named-parameters::Statement] docs for details.
#[proc_macro_derive(Statement, attributes(statement))]
pub fn derive_statement(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    statement::derive_statement_impl(ast)
        .unwrap_or_else(|err| err.into_compile_error())
        .into()
}
