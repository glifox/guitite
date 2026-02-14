use proc_macro_error::abort;
use quote::ToTokens;
use syn::DeriveInput;

pub fn check_fields(tree: &DeriveInput) {
    let fields = match &tree.data {
        syn::Data::Struct(data_struct) => data_struct.fields.clone(),
        syn::Data::Enum(data_enum) => abort!(data_enum.enum_token.to_token_stream(), "Enum type is not supported"),
        syn::Data::Union(data_union) => abort!(data_union.union_token.to_token_stream(), "Union type is not supported"),
    };
    
    let _ = match &fields {
        syn::Fields::Named(fields_named) => fields_named,
        syn::Fields::Unnamed(_) |
        syn::Fields::Unit => abort!(tree.to_token_stream(), "The struct has to implemet a **'doc'** field of type `loro::LoroDoc` and a **'server'** of type `actix::Recipient<guitite::messages::Response>`"),
    };
    
    let mut doc: bool = false;
    let mut server: bool = false;
    for field in &fields {
        let ident = field.ident.clone().unwrap();
        
        let field_type = &field.ty;
        let field_type_tokens = field_type.to_token_stream();
        let string_fielt_type = field_type_tokens.to_string();
        
        if ident == "doc" { 
            doc = true;
            
            if 
                string_fielt_type != "loro :: LoroDoc" &&
                string_fielt_type != "LoroDoc"
            {
                abort!(field_type_tokens, "The **'doc'** field has to be of type `loro::LoroDoc`")
            } 
        }
        
        if ident == "server" {
            server = true; 
            
            if 
                !matches!(field_type, syn::Type::Path(_)) || 
                string_fielt_type != "actix::Recipient < guitite::messages::Response >" && 
                string_fielt_type != "Recipient < guitite::messages::Response >" && 
                string_fielt_type != "actix::Recipient < Response >" && 
                string_fielt_type != "Recipient < Response >"
            {
                abort!(field_type_tokens, "The **'server'** field has to be of type `actix::Recipient<guitite::messages::Response>`")
            } 
        }
    }
    
    if !doc { abort!(fields.to_token_stream(), "The struct has to implemet a **'doc'** field of type `loro::LoroDoc`") }
    if !server { abort!(fields.to_token_stream(), "The struct has to implemet a **'server'** field of type `actix::Recipient<guitite::messages::Response>`") }
}