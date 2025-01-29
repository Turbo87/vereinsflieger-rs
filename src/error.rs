use reqwest::{Response, StatusCode};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The request used an invalid access token or was not authenticated")]
    Unauthorized,
    #[error("The maximum number of requests has been exceeded")]
    TooManyRequests,
    #[error(transparent)]
    RequestFailed(#[from] reqwest::Error),
    #[error(transparent)]
    SerializationFailed(#[from] serde_urlencoded::ser::Error),
    #[error(transparent)]
    DeserializationFailed(#[from] serde_path_to_error::Error<serde_json::Error>),
}

pub async fn error_for_status(response: Response) -> Result<Response> {
    #[derive(Debug, serde::Deserialize)]
    struct ErrorResponse {
        error: String,
    }

    let Err(error) = response.error_for_status_ref() else {
        return Ok(response);
    };

    if error.status() == Some(StatusCode::UNAUTHORIZED) {
        if let Ok(json) = response.json::<ErrorResponse>().await {
            if json.error == "Unauthorized" {
                return Err(Error::Unauthorized);
            }
        }
    } else if error.status() == Some(StatusCode::FORBIDDEN) {
        if let Ok(json) = response.json::<ErrorResponse>().await {
            if json.error.starts_with("Die maximale Anzahl der Requests")
                && json.error.ends_with("ist überschritten!")
            {
                return Err(Error::TooManyRequests);
            }
        }
    }

    Err(error.into())
}
