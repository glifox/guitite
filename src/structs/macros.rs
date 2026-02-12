#[macro_export]
macro_rules! error {
    ($expr:expr) => { log::error!("{:?} in [{}]", $expr, format!("{}:{}:{}", file!(), line!(), column!())) };
}

/// - `invalid` -> Invalid
/// - `no_clients` -> No more clients connect
/// - `un_implemented $action:expr => $mtype:expr` -> Combination un implemented $action: $mtype
/// - `file_not_found` -> __!fatal!__ The server can not found the file any version of the file (You might create one first).
/// - `file_not_found $key` -> __!fatal!__ The server can not found the file: $key
#[macro_export]
macro_rules! errors {
    (invalid) => {{
        let error = crate::structs::messages::Error { status: 404, message: "Invalid".to_string(), fatal: false };
        crate::error!(error);
        error
    }};
    (no_clients) => {{
        let error = crate::structs::messages::Error { status: 404, message: "No more clients connect".to_string(), fatal: false };
        error!(error);
        error
    }};
    (un_implemented $action:expr => $mtype:expr) => {{
        let error = crate::structs::messages::Error { status: 404, message: format!("Combination un implemented {}: {}", $action, $mtype), fatal: false };
        crate::error!(error);
        error
    }};
    (file_not_found) => {{ 
        let error = crate::structs::messages::Error { status: 500, message: format!("The server can not found the file any version of the file (You might create one first)."), fatal: true };
        crate::error!(error);
        error
    }};
    (file_not_found $key:expr) => {{
        let error = crate::structs::messages::Error { status: 500, message: format!("The server can not found the file: {}", $key), fatal: true } ;
        crate::error!(error);
        error
    }};
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

/// `copy msg, mtype, action`
/// `copy msg, action`
#[macro_export]
macro_rules! message {
    (copy $msg:expr, $type:expr, $action:expr) => {
        crate::structs::messages::Message { id: $msg.id.clone(), file: $msg.file.clone(), mtype: $type, action: $action }
    };
    (copy $msg:expr, act $action:expr) => { message!(copy $msg, $msg.mtype.clone(), $action) };
    (copy $msg:expr, mt $type:expr) => { message!(copy $msg, $type, $msg.action.clone()) };
}
