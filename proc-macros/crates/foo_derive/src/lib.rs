extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Foo)]
pub fn my_proc_derive(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = ast.ident;
    let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { named: fields, .. }),
        ..
    }) = ast.data
    else {
        panic!("must be a struct")
    };
    // for each struct field, generate a method to debug it. these methods
    // are token streams and will be interpolated into the result token stream.
    let methods = fields.iter().filter_map(|f| {
        f.ident.as_ref().map(|field_ident| {
            let method_ident = format_ident!("debug_{}", field_ident);
            quote! {
                fn #method_ident(&self) {
                    println!("{} is {}",
                        stringify!(#field_ident),
                        self.#field_ident);
                }
            }
        })
    });
    let expanded = quote! {
        impl #struct_name {
            // interpolate the methods we created for each field.
            #(#methods)*

            // a one-off silly method
            fn say_hello(&self) {
                println!("{}: Hi, {}.", stringify!(#struct_name), self.name);
            }
        }

        // implement the trait
        impl Foo for #struct_name {
            fn foo(&self) -> String {
                format!("foooo from {}", stringify!(#struct_name))
            }
        }
    };
    TokenStream::from(expanded)
}
