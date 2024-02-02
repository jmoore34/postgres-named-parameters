use syn::DeriveInput;

mod numberify;
mod query;
mod util;
mod statement;

#[proc_macro_derive(Query, attributes(query))]
pub fn derive_query(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    query::derive_query_impl(ast)
        .unwrap_or_else(|err| err.into_compile_error())
        .into()
}

#[proc_macro_derive(Statement, attributes(statement))]
pub fn derive_statement(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    statement::derive_statement_impl(ast)
        .unwrap_or_else(|err| err.into_compile_error())
        .into()
}
