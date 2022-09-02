// fn match_for_io_error(err_status: &Status) -> Option<&std::io::Error> {

//     let mut err: &(dyn std::error::Error + 'static) = err_status;

//     loop {
//         if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
//             return Some(io_err);
//         }

//         // h2::Error do not expose std::io::Error with `source()`
//         // https://github.com/hyperium/h2/pull/462
//         if let Some(h2_err) = err.downcast_ref::<h2::Error>() {
//             if let Some(io_err) = h2_err.get_io() {
//                 return Some(io_err);
//             }
//         }

//         err = match err.source() {
//             Some(err) => err,
//             None => return None,
//         };
//     }
// }

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    
    #[error("invalid token")]
    TOKEN_INVALID,
    
    #[error("auth field missing")]
    FIELD_MISSING,
    
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown error")]
    Unknown,
}


#[derive(Error, Debug)]
pub enum TokenError {
    
    #[error("invalid token")]
    TOKEN_INVALID,
    
    #[error("auth field missing")]
    FIELD_MISSING,
    
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum HeaderError {
    
    // #[error("invalid header")]
    // HEADER_ERROR,
    
    #[error("{0} header field missing")]
    FIELD_MISSING(String),
    
    #[error("missing header (expected {expected:?}")]
    MissingHeader {
        expected: String,
    },
    #[error("unknown error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum BodyError {
    
    #[error("Body field {0} missing")]
    FIELD_MISSING(String),

    #[error("unknown error")]
    Unknown,
}







































