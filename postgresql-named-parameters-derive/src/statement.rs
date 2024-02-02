use attribute_derive::FromAttr;
use quote::quote;
use syn::DeriveInput;

#[derive(FromAttr)]
#[attribute(ident = statement)]
struct StatementTraitHelperAttribute {
    #[attribute(example = r#""DELETE FROM Person WHERE id = @id""#)]
    sql: String,
}

pub fn derive_statement_impl(ast: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let args = StatementTraitHelperAttribute::from_attributes(&ast.attrs)?;
    let syn::Data::Struct(struct_ast) = ast.data else {
        return Err(syn::Error::new(
            ast.ident.span(),
            "#[derive(Statement)] can only be used on structs",
        ));
    };

    let named_parameters = crate::util::get_field_names(&struct_ast);
    let parameter_list = crate::util::get_parameter_list(&struct_ast);
    let transformed_sql = match crate::numberify::numberify(args.sql, named_parameters) {
        Ok(sql) => sql,
        Err(err) => {
            let err = format!("Error with SQL provided to #[derive(Statement)]: {}", err);
            return Err(syn::Error::new(ast.ident.span(), err));
        }
    };

    let generics = ast.generics;
    let ident = ast.ident;
    let where_clause = &generics.where_clause;

    let output = quote! {
        #[automatically_derived]
        impl #generics postgresql_named_parameters::Statement for #ident #generics #where_clause {
            fn execute_statement(&self, connection: &mut postgresql_named_parameters::postgres::Client) -> Result<u64, postgresql_named_parameters::postgres::error::Error> {
                connection.execute(#transformed_sql, #parameter_list)
            }
        }
    };
    Ok(output.into())
}
