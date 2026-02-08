#[macro_export]
macro_rules! unwrap_clients_in_file {
    ($self:expr, $msg:expr => $return:expr) => {{
        match $self.get_clients_in_file(&$msg.file) {
            Ok(f) => f,
            Err(err) => {
                $self.send_err(&$msg.sender_id, err);
                return $return
            },
        }
    }};
    ($self:expr, $msg:expr) =>{ unwrap_clients_in_file!($self, $msg => None ) };
}
