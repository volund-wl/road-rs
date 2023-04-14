use async_ffi::FfiFuture;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
#[allow(unused)]
use road_types::{PluginInit, PluginType};
use syn::{parse_macro_input, spanned::Spanned, DeriveInput};

/// This derive macro adds a easy way to init a plugin, this is mostly in use when loading a plugin
/// without the FFI barrier. Do keep in mind that this derive only works with unit structs, if
/// using non unit structs, you'll have to implement PluginInit by yourself
/// ```rs
/// #[derive(Clone, PluginInit)]
/// struct SomePlugin;
///```
#[proc_macro_derive(PluginInit)]
pub fn init_plug(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let expanded = quote! {
        impl PluginInit for #name {
            fn init() -> PluginType<'static> {
                use abi_stable::type_level::downcasting::TD_CanDowncast;
                road_types::Plugin_TO::from_value(Self, TD_CanDowncast)
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}

/// This attribute macro wraps a async function to make it ffi safe.
/// > disclaimer that tokio doesnt
/// work over the binary boundary, so you can't connect the runtimes unless you patch tokio itself
#[proc_macro_attribute]
pub fn async_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    use syn::ReturnType;
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let ret_raw = &input.sig.output;
    let ret = match ret_raw {
        ReturnType::Default => quote!(-> FfiFuture<()>),
        ReturnType::Type(_, t) => {
            let ty = *t.clone();
            quote!(-> FfiFuture<#ty>)
        }
    };
    let inputs = &input.sig.inputs;
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;
    let vis = &input.vis;

    if name == "main" {
        return TokenStream::from(quote_spanned! { name.span() =>
            compile_error!("the main function cannot be tagged with #[road_types::async]"),
        });
    }

    if input.sig.asyncness.is_none() {
        return TokenStream::from(quote_spanned! { input.span() =>
            compile_error!("the async keyword is missing from the function declaration"),
        });
    }

    let result = quote! {
        #(#attrs)*
        #vis fn #name(#inputs) #ret {
            use async_ffi::{FutureExt, FfiFuture};
            let future = async move {
                #body
            };
            future.into_ffi()
        }

    };

    result.into()
}
