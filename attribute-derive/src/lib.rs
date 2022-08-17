extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Data, DeriveInput, Token};

#[proc_macro_derive(Attribute, attributes(attribute))]
pub fn attribute(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let DeriveInput {
        ident, data, attrs, ..
    } = parse_macro_input!(input);

    let attribute = get_attribute(&attrs);

    let AttributeParams(case, input_type, option_type) =
        syn::parse2(attribute.tokens.clone()).expect("Invalid attribute!");

    let is_unit = is_unit(data);

    let constructor = if !is_unit && input_type.is_some() {
        let input_type = input_type.unwrap();
        let converter = match input_type.to_string().as_str() {
            "Option" => match option_type {
                Some(option_type) => {
                    let args = option_type.args;
                    quote! {
                        pub fn new(val: Option<#args>) -> Self {
                            val
                        }
                    }
                }
                None => panic!("Need an option type"),
            },
            "Url" | "String" => {
                quote! {
                    pub fn new(val: #input_type) -> Self {
                        val.to_string()
                    }
                }
            }
            _ => panic!("Unsupported type"),
        };

        quote! {
            impl #ident {
               #converter
            }
        }
    } else {
        quote! {}
    };

    let case = match case.value().as_str() {
        "camelCase" => Case::Camel,
        "kebab-case" => Case::Kebab,
        _ => panic!("Invalid case"),
    };

    let serial = stringify!(ident).to_case(case);

    let output = quote! {
        #constructor

        impl Attribute for #ident {
            fn get_val(&self) -> Option<&str> {
                Some(self.0.as_str())
            }

            fn get_key(&self) -> &str {
                #serial
            }
        }

    };

    output.into()
}

struct AttributeParams(
    syn::LitStr,
    Option<syn::Ident>,
    Option<syn::AngleBracketedGenericArguments>,
);
impl Parse for AttributeParams {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        syn::parenthesized!(content in input);

        let case = content.parse()?;

        match content.parse::<Token![,]>() {
            Ok(_) => (),
            Err(_) => return Ok(AttributeParams(case, None, None)),
        };
        let input_type = content.parse()?;

        match content.parse::<syn::AngleBracketedGenericArguments>() {
            Ok(gen) => Ok(AttributeParams(case, Some(input_type), Some(gen))),
            Err(_) => return Ok(AttributeParams(case, Some(input_type), None)),
        }
    }
}

fn get_attribute<'a>(attrs: &'a Vec<syn::Attribute>) -> &'a syn::Attribute {
    &attrs
        .iter()
        .filter(|a| a.path.segments.len() == 1 && a.path.segments[0].ident == "attribute")
        .nth(0)
        .expect("'attribute' attribute required for deriving Attribute!")
}

fn is_unit(data: Data) -> bool {
    match data {
        syn::Data::Struct(struct_data) => match struct_data.fields {
            syn::Fields::Named(_) => false,
            syn::Fields::Unnamed(_) => false,
            syn::Fields::Unit => true,
        },
        syn::Data::Enum(_) => panic!("Cannot implement Attribute for an Enum"),
        syn::Data::Union(_) => panic!("Cannot implement Attribute for a Union"),
    }
}
