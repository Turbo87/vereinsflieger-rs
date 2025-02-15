use crate::{Article, Credentials, Result, User};
use std::future::Future;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, warn};

/// Client for the Vereinsflieger API.
///
/// This client handles authentication and automatically refreshes the
/// access token when it expires. Due to the implementation details of this
/// client, the requests will be queued up internally and not run in parallel.
///
/// The Vereinsflieger API is rate-limited to one request per second anyway,
/// so this should not be a problem in practice.
#[derive(Debug, Clone)]
pub struct Client {
    /// The internal HTTP client used to make requests.
    client: reqwest::Client,
    /// The current access token, if any.
    access_token: Arc<Mutex<Option<String>>>,
    /// The credentials used to authenticate with the API.
    credentials: Arc<Credentials>,
}

impl Client {
    /// Create a new client with the given credentials.
    pub fn new(credentials: Credentials) -> Self {
        Self {
            client: Default::default(),
            access_token: Default::default(),
            credentials: Arc::new(credentials),
        }
    }

    /// Get the new access token from the API and authenticate with it.
    ///
    /// This does **not** save the access token for future requests! Use
    /// [`set_access_token()`] to save the access token.
    ///
    /// Using this method directly is usually not necessary, but it can be used
    /// to verify that the credentials are correct.
    #[tracing::instrument(skip(self))]
    pub async fn get_access_token(&self) -> Result<String> {
        debug!("Requesting new access token…");
        let access_token = crate::get_access_token(&self.client).await?;

        debug!("Authenticating with new access token…");
        crate::authenticate(&self.client, &access_token, &self.credentials).await?;

        debug!("Authentication successful");
        Ok(access_token)
    }

    /// Save the access token for future requests.
    pub async fn set_access_token(&self, access_token: String) {
        *self.access_token.lock().await = Some(access_token);
    }

    /// Run a request with the current access token, refreshing it if necessary.
    #[tracing::instrument(skip_all)]
    async fn request<T, R, F>(&self, request_fn: F) -> Result<T>
    where
        R: Future<Output = Result<T>>,
        F: Fn(reqwest::Client, String) -> R,
    {
        // Get the current access token, if set.
        let mut access_token_mutex = self.access_token.lock().await;

        // If the access token is set, use it to run the request.
        if let Some(saved_access_token) = access_token_mutex.clone() {
            debug!("Running request with saved access token…");
            let result = request_fn(self.client.clone(), saved_access_token).await;

            // If the request failed with a "401 Unauthorized" error,
            // the access token is invalid and needs to be refreshed.
            //
            // In all other cases, whether the request succeeded or failed,
            // return the result as is.
            if !matches!(result, Err(crate::Error::Unauthorized)) {
                return result;
            }

            debug!("Saved access token is invalid, requesting new access token…");
        }

        // Get a new access token from the API and try to authenticate with it.
        let new_access_token = self.get_access_token().await?;

        // Save the new access token for future requests.
        debug!("Saving access token for future requests…");
        *access_token_mutex = Some(new_access_token.clone());

        // (Re-)run the request with the new access token.
        debug!("Running request with new access token…");
        let result = request_fn(self.client.clone(), new_access_token).await;

        // If the request failed with a "401 Unauthorized" error, the new
        // access token is invalid for some reason and needs to be cleared
        // again.
        if matches!(result, Err(crate::Error::Unauthorized)) {
            warn!("New access token is invalid, clearing saved access token…");
            *access_token_mutex = None;
        }

        // Finally, return the result of the request.
        result
    }

    /// Get the list of all articles from the API.
    #[tracing::instrument(skip_all)]
    pub async fn list_articles(&self) -> Result<Vec<Article>> {
        self.request(|client, access_token| async move {
            crate::list_articles(&client, &access_token).await
        })
        .await
    }

    /// Get the list of all users from the API.
    #[tracing::instrument(skip_all)]
    pub async fn list_users(&self) -> Result<Vec<User>> {
        self.request(|client, access_token| async move {
            crate::list_users(&client, &access_token).await
        })
        .await
    }

    /// Upload a new sale to the API.
    #[tracing::instrument(skip_all)]
    pub async fn add_sale(&self, sale: &crate::NewSale<'_>) -> Result<()> {
        self.request(|client, access_token| async move {
            crate::add_sale(&client, &access_token, sale).await
        })
        .await
    }
}
