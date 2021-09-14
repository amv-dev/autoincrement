extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};

fn get_inner_type(s: &syn::DataStruct) -> syn::Ident {
    if let syn::Fields::Unnamed(fields) = &s.fields {
        let mut fields_iter = fields.unnamed.iter();
        let field = fields_iter.next().expect("Struct must have a single field");

        if fields_iter.next().is_none() {
            if let syn::Type::Path(path) = &field.ty {
                let segs = &path.path.segments;

                if segs.len() != 1 {
                    panic!("Incremental may be implemented only for u8, u16, u32, u64 and usize inner type");
                }

                let seg = segs.first().unwrap();

                match seg.ident.to_string().as_str() {
                    "u8" | "u16" | "u32" | "u64" | "usize" => {
                        return seg.ident.clone();
                    }
                    _ => {}
                }
            }
        }
    }

    do_panic()
}

fn impl_incremental(
    struct_name: &syn::Ident,
    data: &syn::DataStruct,
    attributes: Attributes,
) -> impl ToTokens {
    let inner_type = get_inner_type(data);

    let initial_value = attributes.initial_value;

    quote! {
        impl autoincrement::Incremental for #struct_name {
            fn initial() -> Self {
                Self(#initial_value as #inner_type)
            }

            fn get_next(current: &Self) -> Self {
                let Self(inner) = current;

                Self(inner + 1)
            }
        }

        impl #struct_name {
            pub fn init() -> autoincrement::AutoIncrement<Self> {
                autoincrement::Incremental::init()
            }
        }
    }
}

fn do_panic() -> ! {
    panic!("Incremental attribute available only for struct with a single unnamed field")
}

fn impl_async_incremental(
    struct_name: &syn::Ident,
    data: &syn::DataStruct,
    attributes: Attributes,
) -> impl ToTokens {
    let inner_type = get_inner_type(data);

    let initial_value = attributes.initial_value;

    let atomic = format_ident!("AtomicU{}", format!("{}", inner_type)[1..]);

    quote! {
        impl autoincrement::AsyncIncremental for #struct_name {
            type Atomic = std::sync::atomic::#atomic;

            fn initial() -> Self {
                Self(#initial_value as #inner_type)
            }

            fn get_next(atomic: &Self::Atomic) -> Self {
                Self(atomic.fetch_add(1, std::sync::atomic::Ordering::SeqCst))
            }

            fn into_atomic(value: Self) -> Self::Atomic {
                let Self(inner) = value;
                Self::Atomic::new(inner)
            }
        }

        impl #struct_name {
            pub fn init() -> autoincrement::AsyncIncrement<Self> {
                autoincrement::AsyncIncremental::init()
            }
        }
    }
}

struct Attributes {
    initial_value: u64,
}

impl Default for Attributes {
    fn default() -> Self {
        Self { initial_value: 1 }
    }
}

fn parse_attrs(/*attrs: TokenStream*/) -> Attributes {
    // let parse: syn::DeriveInput = syn::parse(item).expect("Error parsing attributes");
    // attrs.
    Attributes::default()
}

#[proc_macro_derive(Incremental)]
pub fn incremental(item: TokenStream) -> TokenStream {
    // syn::
    let parse: syn::DeriveInput = syn::parse(item).expect("Error parsing struct");

    if parse.generics.params.iter().next().is_some() {
        panic!("Incremental type must not have generics");
    }

    let data = &parse.data;
    let struct_name = &parse.ident;

    let attrs = parse_attrs(); // attrs);

    let impl_struct = match data {
        syn::Data::Struct(s) => impl_incremental(struct_name, s, attrs),
        _ => do_panic(),
    };

    (quote!(#impl_struct)).into()
}

#[proc_macro_derive(AsyncIncremental)]
pub fn async_incremental(item: TokenStream) -> TokenStream {
    let parse: syn::DeriveInput = syn::parse(item).expect("Error parsing struct");

    if parse.generics.params.iter().next().is_some() {
        panic!("Incremental type must not have generics");
    }

    let data = &parse.data;
    let struct_name = &parse.ident;

    let attrs = parse_attrs(); // attrs);

    let impl_struct = match data {
        syn::Data::Struct(s) => impl_async_incremental(struct_name, s, attrs),
        _ => do_panic(),
    };

    (quote! (#impl_struct)).into()
}
