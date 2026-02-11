
/// - `invalid` -> Invalid
/// - `no_clients` -> No more clients connect
/// - `un_implemented $action:expr => $mtype:expr` -> Combination un implemented $action: $mtype
/// - `file_not_found` -> __!fatal!__ The server can not found the file any version of the file (You might create one first).
/// - `file_not_found $key` -> __!fatal!__ The server can not found the file: $key
#[macro_export]
macro_rules! errors {
    (invalid) => { crate::structs::messages::Error { status: 404, message: "Invalid".to_string(), fatal: false } };
    (no_clients) => { crate::structs::messages::Error { status: 404, message: "No more clients connect".to_string(), fatal: false } };
    (un_implemented $action:expr => $mtype:expr) => { crate::structs::messages::Error { status: 404, message: format!("Combination un implemented {}: {}", $action, $mtype), fatal: false } };
    (file_not_found) => { crate::structs::messages::Error { status: 500, message: format!("The server can not found the file any version of the file (You might create one first)."), fatal: true } };
    (file_not_found $key:expr) => { crate::structs::messages::Error { status: 500, message: format!("The server can not found the file: {}", $key), fatal: true } };
}

/// self, msg => return
#[macro_export]
macro_rules! unwrap_clients_in_file {
    ($self:expr, $msg:expr => $return:expr) => {{
        match $self.get_clients_in_file(&$msg.file) {
            Ok(f) => f,
            Err(err) => {
                $self.send_err(&$msg.id, err);
                return $return
            },
        }
    }};
    ($self:expr, $msg:expr) =>{ unwrap_clients_in_file!($self, $msg => () ) };
}

#[macro_export]
macro_rules! message {
    (copy $msg:expr, $type:expr, $action:expr) => {
        crate::structs::messages::Message { id: $msg.id.clone(), file: $msg.file.clone(), mtype: $type, action: $action }
    };
}
