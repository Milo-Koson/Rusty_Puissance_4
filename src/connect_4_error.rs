use std::fmt;
use std::sync::mpsc::SendError;
use crate::EventTimerTick;

pub type Connect4Result<T> = Result<T, Connect4Error>;

/**
Utiliser Connect4Error :
fct_avec_resultat()
    .ok_or(Connect4Result::[valeur_enum])

NB: Retourne le type Connect4Result si erreur avec le bon enum.
Il est possible d'utiliser and_then() pour effectuer des instructions
*/

#[derive(Debug)]
pub enum Connect4Error {
    ChannelRecv,
    ChannelSend,
    GraphicalTimerError,
    InvalidInput,
    ColumnFull
}

impl fmt::Display for Connect4Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Connect4Error::ChannelRecv =>
            write!(f, "[CONNECT_4_ERROR] - Channel Recv"),
            Connect4Error::ChannelSend =>
                write!(f, "[CONNECT_4_ERROR] - Channel Send"),
            Connect4Error::InvalidInput =>
                write!(f, "[CONNECT_4_ERROR] - Input Error"),
            Connect4Error::ColumnFull =>
                write!(f, "[CONNECT_4_ERROR] - Column is full"),
            _ =>
                write!(f, "[CONNECT_4_ERROR] - huh ?"),
        }
    }
}

impl From<SendError<EventTimerTick>> for Connect4Error {
    fn from(value: SendError<EventTimerTick>) -> Self {
        match value {
            SendError(_) => Connect4Error::ChannelSend
        }
    }
}

