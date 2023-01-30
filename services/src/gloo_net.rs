use gloo_net::http::Method;
use serde::{de::DeserializeOwned, Serialize};

use crate::error::Error;
use types::ErrorInfo;

// consts and token-management were move to the super module.

/// build all kinds of http request: post/get/delete etc.
pub async fn request<B, T>(method: Method, url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    let allow_body = matches!(method, Method::POST) || matches!(method, Method::PUT);
    let url = format!("{}{}", super::API_ROOT, url);

    let mut req = gloo_net::http::Request::new(&url)
        .method(method)
        .header("Content-Type", "application/json");
    if let Some(token) = super::get_token() {
        req = req.header("Authorization", &format!("Token {}", token));
    }

    if allow_body {
        req = req.json(&body).map_err(|_| Error::RequestError)?;
    }

    let response = req.send().await;

    if let Ok(data) = response {
        if data.status() == 200 {
            let data: Result<T, _> = data.json::<T>().await;
            if let Ok(data) = data {
                // log::debug!("Response: {:?}", data);
                Ok(data)
            } else {
                Err(Error::DeserializeError)
            }
        } else {
            let status = data.status();
            let error = match data.json::<ErrorInfo>().await {
                Ok(data) => Error::UnprocessableEntity(data),
                Err(_) => Error::DeserializeError,
            };
            Err(Error::from_status_code(status, error))
        }
    } else {
        Err(Error::RequestError)
    }
}

/// Delete request
pub async fn request_delete<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(Method::DELETE, url, ()).await
}

/// Get request
pub async fn request_get<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(Method::GET, url, ()).await
}

/// Post request with a body
pub async fn request_post<B, T>(url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    request(Method::POST, url, body).await
}

/// Put request with a body
pub async fn request_put<B, T>(url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    request(Method::PUT, url, body).await
}

