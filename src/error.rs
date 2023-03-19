use crate::color::ColorError;
use std::fmt;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    LightNotFound,
    BluetoothError(#[from] btleplug::Error),
    Color(#[from] ColorError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        let message = match self {
            LightNotFound => "Light Not Found".into(),
            BluetoothError(e) => format!("{}", e),
            Color(e) => format!("{}", e),
        };

        write!(f, "{}", message)
    }
}
