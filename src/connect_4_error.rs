use std::fmt;
use std::sync::mpsc::SendError;
use crate::EventTimerTick;

/**
Type du résultat du jeu comprenant le type à renvoyer en cas de réussite sans erreur (T, qui se
remplace par le type voulu) et une énumération d'erreur du jeu avec le type de l'erreur (Connect4Error).
*/
pub type Connect4Result<T> = Result<T, Connect4Error>;

/**
Enumeration d'erreur possible dans le jeu.
*/
#[derive(Debug)]
pub enum Connect4Error {
    /**
    Erreur dans un canal de communication en réception.
    */
    ChannelRecv,
    /**
    Erreur dans un canal de communication en envoi.
     */
    ChannelSend,
    /**
    Erreur d'une saisi de l'utilisateur invalide.
     */
    InvalidInput,
    /**
    Erreur d'une colonne complète.
     */
    ColumnFull
}

/**
Implémentation de Display de fmt pour afficher l'erreur en console.
*/
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
        }
    }
}

/**
Implémentation de SendError de From pour l'envoi de l'énumération EventTimerTick dans un canal de
communication.
*/
impl From<SendError<EventTimerTick>> for Connect4Error {
    fn from(value: SendError<EventTimerTick>) -> Self {
        match value {
            SendError(_) => Connect4Error::ChannelSend
        }
    }
}

