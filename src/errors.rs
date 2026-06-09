use std::{error, fmt, io};

#[derive(Debug)]
pub enum Error {
    LineParse(String, usize),
    Io(io::Error),
    EnvVar(std::env::VarError),
    #[doc(hidden)]
    __Nonexhaustive
}

impl Error {
    pub fn not_found(&self) -> bool {
        if let Error::Io(ref io_error) = *self {
            return io_error.kind() == io::ErrorKind::NotFound;
        }
        false
    }
}

impl error::Error for Error {
    fn source(&self ) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            Error::EnvVar(err) => Some(err),
            _ => None
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(err) => write!(fmt, "{}", err),
            Error::EnvVar(err)  => write!(fmt, "{}", err),
            Error::LineParse(line, error_index ) => write!(
                fmt,
                "Error parsing line: '{}', error at line index: {}",
                line, error_index
            ),
            _ => unreachable!()
        }
    }
}