use super::internal::{Action, MessageType};

#[allow(dead_code)]
trait Parser {
    type Parameter;
    type Return;
    fn parse(param: &Self::Parameter) -> Result<Self::Return, &'static str> ;
}

pub(crate) trait Parse {
    type Return;
    fn parse(&self) -> Result<Self::Return, &'static str> ;
}

impl Parser for Action {
    type Parameter = u8;
    type Return = Self;

    fn parse(param: &Self::Parameter) -> Result<Self::Return, &'static str>  {
        match param {
            0 => Ok(Action::None),
            1 => Ok(Action::Replicate),
            2 => Ok(Action::Answer),
            3 => Ok(Action::Passthrough),
            _ => Err("Invalid Action")
        }
    }
}

impl Parser for MessageType {
    type Parameter = (u8, Vec<u8>);
    type Return = Self;

    fn parse(param: &Self::Parameter) -> Result<Self::Return, &'static str>  {
        let (type_, content) = param.clone();
        
        match type_ {
            0 => Ok(MessageType::None),
            1 => Ok(MessageType::Export(content)),
            2 => Ok(MessageType::VersionVector(content)),
            3 => Ok(MessageType::Frontiers(content)),
            4 => Ok(MessageType::Ephimeral(content)),
            _ => Err("Invalid Message type")
        }
    }
}

impl Parse for Vec<u8> {
    type Return = (MessageType, Action);
    fn parse(&self) -> Result<Self::Return, &'static str> {
        let arr: [u8; 2] = match self {
            vec if vec.is_empty() => return Err("vacio"),
            vec if vec.len() < 2 => return Err("empty"),
            vec => vec[..2].try_into().unwrap()
        };
        
        let vec: Vec<u8> = (self[2..]).to_vec();
        let action = Action::parse(&arr[1])?;
        let message = MessageType::parse(&(arr[0], vec))?;
        
        Ok((message, action))
    }
}
