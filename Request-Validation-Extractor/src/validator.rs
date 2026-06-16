use axum::{
    Json,
    extract::{FromRequest, Request, rejection::JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::{Validate, ValidationErrors};

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] ValidationErrors),
    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),
}

// handling the error to be sent as proper responses
impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(_) => {
                let message = format!("Input Validation Error [{self}]").replace("\n", ",");
                (StatusCode::BAD_REQUEST, message).into_response()
            }
            _ => (StatusCode::BAD_REQUEST, self.into_response()).into_response(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedPayload<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedPayload<T>
where
    T: DeserializeOwned + Validate + Send,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = ServerError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?; // getting the json struct from the request (deserializing)
        value.validate()?; // this is telling the validate lib to read the validate macro in the struct
        Ok(ValidatedPayload(value))
    }
}
