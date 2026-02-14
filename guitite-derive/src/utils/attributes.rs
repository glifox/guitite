use proc_macro_error::abort;
use syn::DeriveInput;
use syn::Ident;
use syn::Token;
use syn::parse::Parse;

#[derive(Debug)]
pub struct Skips {
    pub connect: bool,
    pub message: bool,
    pub disconnect: bool,
}

impl Default for Skips {
    fn default() -> Self { Self { connect: false, message: false, disconnect: false } }
}

impl Skips {
    fn values() ->[&'static str; 3] {
        ["skip_connect", "skip_message", "skip_disconnect"]
    }
    pub fn from(tree: &DeriveInput) -> Self {
        match tree.attrs.iter().find_map(|a| {
            if let Some(ident) = a.meta.path().get_ident() && ident == "document_actor" {
                Some(a)
            } 
            else { None }
        }) {
            Some(a) => a.parse_args().unwrap(),
            None => return Self::default(),
        }
    }
}

impl Parse for Skips {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut _self = Self::default();
        
        if input.is_empty() { return Ok(_self) }
        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            match ident.to_string().as_str() {
                "skip_connect" => {
                    if _self.connect { abort!(ident, "the atribute must apear exactly one time") }
                    _self.connect = true;
                },
                "skip_message" => {
                    if _self.message { abort!(ident, "the atribute must apear exactly one time") }
                    _self.message = true;
                },
                "skip_disconnect" => {
                    if _self.disconnect { abort!(ident, "the atribute must apear exactly one time") }
                    _self.disconnect = true;
                },
                _ => abort!(ident, "unexpected attribute, posible values are {:?}", Self::values())
            }
            if input.peek(Token![,]) { input.parse::<Token![,]>()?; }
            else if !input.is_empty() { 
                abort!(input.span(), "expected ',', got {}", input) 
            }
        };
        
        Ok(_self)
    }
}