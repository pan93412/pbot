use quote::quote;
/// PBot: Procedural Macros
///
/// This includes macros for procedural macros
/// for making modules and some reuse part.
use syn::{parse::Parse, parse_macro_input, DeriveInput, Token};

/// The `ModuleActivator` derive macro.
///
/// # Example
///
/// ```ignore
/// use pbot_modules_derive::ModuleActivator;
///
/// #[derive(ModuleActivator)]
/// pub struct YourModuleActor;
///
/// // -> impl crate::modules::base::ModuleActivator for YourModuleActor {}
/// ```
#[proc_macro_derive(ModuleActivator)]
pub fn derive_module_activator(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    let impl_blk = quote! {
        impl crate::modules::base::ModuleActivator for #ident {}
    };

    impl_blk.into()
}

/// The `ModuleActivator` derive macro.
///
/// # Example
///
/// ```ignore
/// use pbot_modules_derive::ModuleActor;
///
/// #[derive(ModuleActor)]
///
/// pub struct YourModuleActor;
///
/// // -> impl Actor for YourModuleActor { ... }
/// ```
#[proc_macro_derive(ModuleActor)]
pub fn derive_module_actor(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    quote! {
        impl Actor for #ident {
            type Context = Context<Self>;

            fn started(&mut self, _: &mut Self::Context) {
                use crate::modules::base::ModuleMeta;

                info!("🌟 {} started!", self.name());
            }

            fn stopped(&mut self, _: &mut Self::Context) {
                use crate::modules::base::ModuleMeta;

                info!("👋 {} stopped.", self.name());
            }
        }
    }.into()
}

/// The name part of a module metadata attribute.
///
/// ```ignore
/// # use pbot_modules_derive::ModuleMeta;
/// 
/// #[derive(ModuleMeta)]
/// #[name = "YourModule"]
/// pub struct Module;
/// ```
#[proc_macro_derive(ModuleMeta, attributes(name))]
pub fn derive_module_meta(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    let name = input
        .attrs
        .into_iter()
        .find(|attr| attr.path.is_ident("name"))
        .map(|attr| {
            (syn::parse2(attr.tokens) as Result<MetaNameAttribute, _>).expect("must be parsable")
        })
        .expect("must have a name attribute")
        .0;

    quote! {
        impl crate::modules::base::ModuleMeta for #ident {
            fn name(&self) -> &'static str {
                #name
            }
        }
    }
    .into()
}

/// The attribute of `#[name = "..."]` in ModuleMeta.
struct MetaNameAttribute(syn::LitStr);

impl Parse for MetaNameAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Token![=]>()?;
        let val = input.parse()?;

        Ok(MetaNameAttribute(val))
    }
}
