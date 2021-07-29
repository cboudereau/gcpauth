#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("json parse error")]
    JsonError(#[from] json::Error),

    #[error("error `{0}`")]
    StringError(String),

    #[error("jwt error")]
    JwtError(#[from] jwt::errors::Error),

    #[error("hyper error")]
    HyperError(#[from] hyper::Error),

    #[error("io error")]
    IOError(#[from] std::io::Error),
}
