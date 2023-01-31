use crate::error::Error;
use gloo_net::http::{Method, Request as GlooRequest};

pub struct Request(Option<GlooRequest>);

fn request<B: serde::Serialize>(method: gloo_net::http::Method, url: &str, body: &B) -> Request {
    let url = format!("{}{}", super::API_ROOT, url);
    let builder = GlooRequest::new(&url)
        .method(method)
        .header("Content-Type", "application/json");
    let req = Request(Some(builder)).set_token();
    Request(req.0.and_then(|r| r.json(body).ok()))
}

impl Request {
    fn set_token(self) -> Self {
        let Some(r) = self.0 else {
            return Self(None);
        };
        let builder = match super::get_token() {
            Some(token) => r.header("Authorization", &format!("Token {}", token)),
            None => r,
        };
        Self(Some(builder))
    }

    pub async fn send<T: serde::de::DeserializeOwned>(self) -> Result<T, Error> {
        let Some(r) = self.0 else {
            return Err(Error::RequestError);
        };
        let response = r.send().await;

        if let Ok(data) = response {
            if data.status() == 200 {
                let data: Result<T, _> = data.json::<T>().await;
                if let Ok(data) = data {
                    Ok(data)
                } else {
                    Err(Error::DeserializeError)
                }
            } else {
                let status = data.status();
                let error = match data.json::<types::ErrorInfo>().await {
                    Ok(data) => Error::UnprocessableEntity(data),
                    Err(_) => Error::DeserializeError,
                };
                Err(Error::from_status_code(status, error))
            }
        } else {
            Err(Error::RequestError)
        }
    }
}

pub fn request_delete(url: &str) -> Request {
    Request(Some(GlooRequest::new(url).method(Method::DELETE))).set_token()
}

pub fn request_get(url: &str) -> Request {
    Request(Some(GlooRequest::new(url).method(Method::GET))).set_token()
}

pub fn request_post<B>(url: &str, body: &B) -> Request
where
    B: serde::Serialize,
{
    request(Method::POST, url, body)
}

pub fn request_put<B>(url: &str, body: &B) -> Request
where
    B: serde::Serialize,
{
    request(Method::PUT, url, body)
}
