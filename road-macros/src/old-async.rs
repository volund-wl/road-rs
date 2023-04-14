/// This attribute macro wraps a async function to make it ffi safe.
/// > disclaimer that tokio doesnt
/// work over the binary boundary, so you can't connect the runtimes unless you patch tokio itself
#[proc_macro_attribute]
pub fn async_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    use syn::ReturnType;
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let ret_raw = &input.sig.output;
    let ret = match ret_raw {
        ReturnType::Default => quote!(-> BoxFuture<'static, ()>),
        ReturnType::Type(_, t) => {
            let ty = *t.clone();
            quote!(-> BoxFuture<'static, #ty>)
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
            use std::future::IntoFuture;
            let future = async move {
                #body
            };
            BoxFuture::new(future.into_future())
        }

    };

    result.into()
}
