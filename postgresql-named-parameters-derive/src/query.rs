use attribute_derive::FromAttr;
use quote::quote;
use syn::{DeriveInput, Type};

#[derive(FromAttr)]
#[attribute(ident = query)]
struct QueryTraitHelperAttribute {
    #[attribute(example = r#""SELECT * FROM Person WHERE first_name = @name""#)]
    sql: String,
    #[attribute(example = "crate::my_database_tables::Person")]
    row: Type,
}

pub fn derive_query_impl(ast: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let args = QueryTraitHelperAttribute::from_attributes(&ast.attrs)?;
    let syn::Data::Struct(struct_ast) = ast.data else {
        return Err(syn::Error::new(
            ast.ident.span(),
            "#[derive(Query)] can only be used on structs",
        ));
    };
    let named_parameters = crate::util::get_field_names(&struct_ast);
    let parameter_list = crate::util::get_parameter_list(&struct_ast);
    let transformed_sql = match crate::numberify::numberify(args.sql, named_parameters) {
        Ok(sql) => sql,
        Err(err) => {
            let err = format!("Error with SQL provided to #[derive(Query)]: {}", err);
            return Err(syn::Error::new(ast.ident.span(), err));
        }
    };

    let generics = ast.generics;
    let ident = ast.ident;
    let where_clause = &generics.where_clause;
    let row_type = args.row;

    let output = quote! {
        #[automatically_derived]
        impl #generics postgresql_named_parameters::Query for #ident #generics #where_clause {
            type Row = #row_type;
            fn query_all(
                &self,
                connection: &mut impl postgresql_named_parameters::postgres::GenericClient,
            ) -> Result<Vec<Self::Row>, postgresql_named_parameters::postgres::error::Error> {
                let rows = connection.query(#transformed_sql, #parameter_list)?;
                rows
                    .iter()
                    .map(postgresql_named_parameters::internal::wrapper_for_derive_macro::try_from_row::<Self::Row>)
                    .collect()
            }

            fn query_opt(
                &self,
                connection: &mut impl postgresql_named_parameters::postgres::GenericClient,
            ) -> Result<Option<Self::Row>, postgresql_named_parameters::postgres::error::Error> {
                let maybe_row = connection.query_opt(#transformed_sql, #parameter_list)?;
                match maybe_row {
                    None => Ok(None),
                    Some(row) => {
                        let decoded_row = postgresql_named_parameters::internal::wrapper_for_derive_macro::try_from_row::<Self::Row>(&row)?;
                        Ok(Some(decoded_row))
                    }
                }
            }

            fn query_one(
                &self,
                connection: &mut impl postgresql_named_parameters::postgres::GenericClient,
            ) -> Result<Self::Row, postgresql_named_parameters::postgres::error::Error> {
                let row = connection.query_one(#transformed_sql, #parameter_list)?;
                postgresql_named_parameters::internal::wrapper_for_derive_macro::try_from_row::<Self::Row>(&row)
            }
        }
    };
    Ok(output.into())
}
