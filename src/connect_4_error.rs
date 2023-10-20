use std::fmt;

pub type Connect4Result<T> = Result<T, Connect4Error>;

/**
Utiliser Connect4Error :
fct_avec_resultat()
    .ok_or(Connect4Result::[valeur_enum])

NB: Retourne le type Connect4Result si erreur avec le bon enum.
Il est possible d'utiliser and_then() pour effectuer des instructions
*/
/*
#[derive(Debug)]
pub struct Connect4Message {
    connect_4_message: String
}*/

#[derive(Debug)]
pub enum Connect4Error {
    Ok,
    ChannelRecv,
    KeyboardRecvPass,
    KeyboardRecvUnknown,
    AwaitError,
    ErrorUnknown,
    GraphicalError
}

impl fmt::Display for Connect4Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Connect4Error::ChannelRecv =>
            write!(f, "[CONNECT_4_ERROR] - Channel Recv"),
            Connect4Error::KeyboardRecvUnknown =>
                write!(f, "[CONNECT_4_ERROR] - Keyboard Recv Unknown"),
            Connect4Error::KeyboardRecvPass =>
                write!(f, "[CONNECT_4_ERROR] - Keyboard Recv Pass"),
            Connect4Error::ErrorUnknown =>
                write!(f, "[CONNECT_4_ERROR] - Error Unknown"),
            _ =>
                write!(f, "[CONNECT_4_ERROR] - huh ?"),
        }
    }
}

