use quote::quote;
use syn::DeriveInput;

mod numberify;

#[proc_macro_derive(Query, attributes(sql))]
pub fn derive_query(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let sql_input = {
        let attribute = ast
            .attrs
            .iter()
            .find(|attribute|
                attribute
                .path()
                .is_ident("sql")
            )
            .expect(
                r#"In order to use #[derive(Query)] on a struct, you must also put #[sql = "some SQL query"] on the struct."#
            );

        let err = Err(
            r#"The provided query must be a string literal. Example usage: #[sql = "SELECT * FROM some_table"]"#
        );
        if let syn::Meta::NameValue(name_value_attribute) = &attribute.meta {
           if let syn::Expr::Lit(literal) = &name_value_attribute.value {
                if let syn::Lit::Str(string) = &literal.lit {
                    Ok(string.value())
                } else {err}
            } else {err}
        } else {err}
    }.unwrap();

    let syn::Data::Struct(struct_ast) = ast.data else {
        panic!("#[derive(Query)] can only be used on structs");
    };

    let named_parameters = match struct_ast.fields {
        syn::Fields::Unnamed(_) => vec![],
        syn::Fields::Unit => vec![],
        syn::Fields::Named(ref named_fields) => get_field_identifiers(named_fields)
            .iter()
            .map(|f| f.to_string())
            .collect(),
    };

    let parameter_list = match struct_ast.fields {
        syn::Fields::Unit => quote!(&[]),
        syn::Fields::Named(ref named_fields) => {
            let field_names = get_field_identifiers(named_fields);
            quote! {
                &[
                    #(&self.#field_names),*
                ]
            }
        }
        syn::Fields::Unnamed(unnamed_fields) => {
            let indices = (0..unnamed_fields.unnamed.len()).map(syn::Index::from);
            quote! {
                &[
                    #( self.#indices ),*
                ]
            }
        }
    };

    let interpolated_query = numberify::numberify(sql_input, named_parameters).unwrap();

    let generics = ast.generics;
    let ident = ast.ident;
    let where_clause = &generics.where_clause;
    // panic!("{interpolated_query}");

    quote! {
        #[automatically_derived]
        impl #generics postgresql_named_parameters::Query for #ident #generics #where_clause {
            fn sql() -> &'static str {
                #interpolated_query
            }

            fn parameter_names() -> &'static str {
                stringify!(#parameter_list)
            }

            fn execute(&self, connection: &mut postgres::Client) -> Result<u64, postgres::error::Error> {
                connection.execute(#interpolated_query, #parameter_list)
            }
        }
    }.into()
}

fn get_field_identifiers(named_fields: &syn::FieldsNamed) -> Vec<proc_macro2::Ident> {
    named_fields
        .named
        .iter()
        .filter_map(|named_field| named_field.ident.as_ref().map(|i| i.to_owned()))
        .collect()
}
