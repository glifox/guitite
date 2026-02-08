
/// `invalid` -> Invalid
/// 
/// `no_clients` -> No more clients connect
/// 
/// `un_implemented $action:expr => $mtype:expr` -> Combination un implemented $action: $mtype
/// 
/// `file_not_found $key` -> !! The server can not found the file: $key
/// 
#[macro_export]
macro_rules! errors {
    (invalid) => { crate::structs::messages::Error { status: 404, message: "Invalid".to_string(), fatal: false } };
    (no_clients) => { crate::structs::messages::Error { status: 404, message: "No more clients connect".to_string(), fatal: false } };
    (un_implemented $action:expr => $mtype:expr) => { crate::structs::messages::Error { status: 404, message: format!("Combination un implemented {}: {}", $action, $mtype), fatal: false } };
    (file_not_found $key:expr) => { crate::structs::messages::Error { status: 500, message: format!("The server can not found the file: {}", $key), fatal: true } };
}