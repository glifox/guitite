use quote::quote;
use syn::{Generics, Ident};


pub fn tokens(name: &Ident, generics: &Generics) -> proc_macro2::TokenStream {
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    
    quote! {
        impl #impl_generics actix::Handler<guitite::messages::Connect> for #name #type_generics #where_clause
        {
            type Result = ();
            
            fn handle(&mut self, msg: guitite::messages::Connect, _: &mut Self::Context) -> Self::Result {
                let version = self.doc.state_vv().encode();
                msg.addr_msg.do_send(guitite::message!(copy msg, guitite::types::MessageType::VersionVector(version), guitite::types::Action::Answer));
            }
        }
    }
}
