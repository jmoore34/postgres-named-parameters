use quote::quote;

pub fn get_field_names(struct_ast: &syn::DataStruct) -> Vec<String> {
    match struct_ast.fields {
        syn::Fields::Unnamed(_) => vec![],
        syn::Fields::Unit => vec![],
        syn::Fields::Named(ref named_fields) => get_field_identifiers(named_fields)
            .iter()
            .map(|f| f.to_string())
            .collect(),
    }
}

pub fn get_parameter_list(struct_ast: &syn::DataStruct) -> proc_macro2::TokenStream {
    match &struct_ast.fields {
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
                    #(&self.#indices),*
                ]
            }
        }
    }
}

fn get_field_identifiers(named_fields: &syn::FieldsNamed) -> Vec<proc_macro2::Ident> {
    named_fields
        .named
        .iter()
        .filter_map(|named_field| named_field.ident.as_ref().map(|i| i.to_owned()))
        .collect()
}