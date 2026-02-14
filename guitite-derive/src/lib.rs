mod utils;

use proc_macro_error::proc_macro_error;
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::utils::attributes::Skips;
use crate::utils::cheks::check_fields;
use crate::utils::impl_connect;
use crate::utils::impl_disconnect;
use crate::utils::impl_message;



fn implementation(tree: DeriveInput, skips: Skips) -> TokenStream {
    check_fields(&tree);

    let name = tree.ident;
    let generics = &tree.generics;
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    
    let connect_impl = if skips.connect { quote! {  } } else { impl_connect::tokens(&name, generics) };
    let message_impl = if skips.message { quote! {  } } else { impl_message::tokens(&name, generics) };
    let disconn_impl = if skips.disconnect { quote! {  } } else { impl_disconnect::tokens(&name, generics) };
    
    quote! {
        #[allow(dead_code)]
        const _: () = {
            // this checks the implementation of the `guitite::Protocol` for the struct
            struct ProtocolCheck<T: guitite::Protocol>(std::marker::PhantomData<T>);
            let _: ProtocolCheck::<#name>;
        };
        
        impl #impl_generics #name #type_generics #where_clause { 
            fn do_send(&self, msg: guitite::messages::Response) { 
                log::debug!("[>]  sended: {:?}", msg);
                self.server.do_send(msg);
            }
        }
        
        impl #impl_generics actix::Actor for #name #type_generics #where_clause { type Context = actix::Context<Self>; }
        
        #connect_impl
        #message_impl
        #disconn_impl
    }.into()
}


#[proc_macro_derive(DocumentActor, attributes(document_actor))]
#[proc_macro_error]
pub fn document_actor(input: TokenStream) -> TokenStream {
    let input2: proc_macro2::TokenStream = input.into(); 
    let tree: DeriveInput = syn::parse2(input2).unwrap();
    
    let skips = Skips::from(&tree);
    implementation(tree, skips)
}

#[proc_macro_derive(Protocol)]
#[proc_macro_error]
pub fn protocol(input: TokenStream) -> TokenStream {
    let input2: proc_macro2::TokenStream = input.into(); 
    let tree: DeriveInput = syn::parse2(input2).unwrap();
    
    let name = tree.ident;
    let (impl_generics, type_generics, where_clause) = tree.generics.split_for_impl();
    
    quote! {
        impl #impl_generics guitite::Protocol for #name #type_generics #where_clause { }
    }.into()
}
