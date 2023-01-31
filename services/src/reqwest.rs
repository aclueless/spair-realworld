use crate::error::Error;

pub struct Request(reqwest::RequestBuilder);

fn request<B: serde::Serialize>(method: reqwest::Method, url: &str, body: &B) -> Request {
    let url = format!("{}{}", super::API_ROOT, url);
    let builder = reqwest::Client::new()
        .request(method, url)
        .header("Content-Type", "application/json");
    let req = Request(builder).set_token();
    Request(req.0.json(body))
}

impl Request {
    fn set_token(self) -> Self {
        let builder = match super::get_token() {
            Some(token) => self.0.bearer_auth(token),
            None => self.0,
        };
        Self(builder)
    }

    pub async fn send<T: serde::de::DeserializeOwned>(self) -> Result<T, Error> {
        let response = self.0.send().await;

        if let Ok(data) = response {
            if data.status().is_success() {
                let data: Result<T, _> = data.json::<T>().await;
                if let Ok(data) = data {
                    Ok(data)
                } else {
                    Err(Error::DeserializeError)
                }
            } else {
                let status = data.status().as_u16();
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
    Request(reqwest::Client::new().delete(url)).set_token()
}

pub fn request_get(url: &str) -> Request {
    Request(reqwest::Client::new().get(url)).set_token()
}

pub fn request_post<B>(url: &str, body: &B) -> Request
where
    B: serde::Serialize,
{
    request(reqwest::Method::POST, url, body)
}

pub fn request_put<B>(url: &str, body: &B) -> Request
where
    B: serde::Serialize,
{
    request(reqwest::Method::PUT, url, body)
}
