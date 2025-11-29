use proc_macro::{TokenStream};
use quote::format_ident;
use syn::{Data, DeriveInput, Field, Fields, Token};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Token};
use deluxe;


#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(register))]
struct RegisterIDAttribute {
    #[deluxe(default = None)]
    id : Option<()>
}

fn extract_primary_id_field(fields: &mut Punctuated<Field, Comma>) -> deluxe::Result<&mut Field> {
    let mut found : Option<&mut Field> = None;
    for f in fields.iter_mut() {
        let RegisterIDAttribute {id} = deluxe::extract_attributes(f)?;
        match id {
            Some(_) => match found {
                None => found = Some(f),
                Some(_) => panic!("There can only be one register(id) field")
            },
            None => {}
        }
    };

    Ok(found.expect("No required register(id) field found"))
}

fn impl_register (mut ast : DeriveInput) -> deluxe::Result<proc_macro::TokenStream> {
    let ident = ast.ident;

    // Extract fields from original struct
    let fields = match &mut ast.data {
        Data::Struct(s) => match &mut s.fields {
            Fields::Named(f) => &mut f.named,
            _ => panic!("Register only supports structs with named fields"),
        },
        _ => panic!("Register only supports structs"),
    };

    let id_field = extract_primary_id_field(fields)?;
    let id_field_type = &id_field.ty;
    let id_field_ident = id_field.ident.as_ref().unwrap();

    let parsed: proc_macro2::TokenStream = quote::quote! {
        const _ : () = {
            extern crate serde;
            extern crate register;

            type StaticRegistry<T> = std::sync::LazyLock<std::sync::RwLock<std::collections::HashMap
                    <#id_field_type, std::sync::Arc<T>>
            >>;
            impl HasRegistry<#id_field_type> for #ident {
                fn get_registry() -> &'static StaticRegistry<#ident> {
                    static MAP : StaticRegistry<#ident> =
                        LazyLock::new(||
                            {RwLock::new(HashMap::<#id_field_type,Arc<#ident>>::new())}
                        );

                    &MAP
                }
                fn get_raw_key(&self) -> #id_field_type {
                    self.#id_field_ident
                }
                fn deserialize_and_register<'de,D>(deserializer: D) -> Result<Arc<Self>, D::Error>
                where
                    D: serde::Deserializer<'de>,
                    Self : serde::Deserialize<'de>
                {
                    // Deserialize
                    let _val = Self::deserialize(deserializer)?;

                    // Move helper into the real struct
                    let mut _ident_value: #ident = _val.into();
                    let id = _ident_value.#id_field_ident.clone();

                    // Automatically call register()
                    register::register_item(_ident_value.#id_field_ident,_ident_value);
                    Ok(#ident::get_from_reg(id).expect("[Internal Error] : Could not load item from register"))
                }
            }
        };
    }.into();
    Ok(parsed.into())
}

#[proc_macro_derive(Register,attributes(register))]
pub fn register_derive_macro(item : proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast : DeriveInput = syn::parse(item.into()).unwrap();
    impl_register(ast).unwrap()
}