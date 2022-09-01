use proc_macro::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Data, DeriveInput,
};
#[macro_use]
extern crate quote;

#[proc_macro_derive(Event, attributes(event_type))]
pub fn event(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, data, attrs, ..
    } = parse_macro_input!(input);

    let field_type = get_field_type(&data);

    let attribute = get_attribute(&attrs);
    let EventType(event_type) = syn::parse2(attribute.tokens.clone()).expect("Invalid attribute!");

    let serial = ident.to_string().to_lowercase();

    let output = quote! {
        impl #ident {
            pub const KEY: &'static str = #serial;

            pub fn new(cb: Box<dyn Fn(&#event_type)>) -> Self {
                let func = move |e: &web_sys::Event| {
                    let event = e.clone();
                    cb(event.dyn_into::<#event_type>().unwrap().as_ref());
                };
                Self(Rc::new(func))
            }
        }

        impl Event for #ident {
            fn get_event_type(&self) -> &str {
                Self::KEY
            }

            fn get_callback(&self) -> #field_type {
                self.0.clone()
            }
        }
    };

    output.into()
}

fn get_field_type(data: &Data) -> &syn::Type {
    let field_type = match &data {
        syn::Data::Struct(struct_data) => match &struct_data.fields {
            syn::Fields::Named(_) => panic!(
                "Can only implement Event on a tuple struct. This is a struct with named fields."
            ),
            syn::Fields::Unnamed(data) => &data.unnamed.first().expect("there to be a field").ty,
            syn::Fields::Unit => {
                panic!("Can only implement Event on a tuple struct. This is a struct unit struct.")
            }
        },
        syn::Data::Enum(_) => panic!(
            "Can only implement Event for a struct. You are attempting to implement it on an Enum"
        ),
        syn::Data::Union(_) => panic!(
            "Can only implement Event for a struct. You are attempting to implement it on a Union"
        ),
    };

    field_type
}

struct EventType(syn::Path);

impl Parse for EventType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        syn::parenthesized!(content in input);

        let event_type: syn::Path = content.parse()?;

        Ok(EventType(event_type))
    }
}

fn get_attribute<'a>(attrs: &'a Vec<syn::Attribute>) -> &'a syn::Attribute {
    &attrs
        .iter()
        .filter(|a| a.path.segments.len() == 1 && a.path.segments[0].ident == "event_type")
        .nth(0)
        .expect("'attribute' attribute required for deriving Attribute!")
}
