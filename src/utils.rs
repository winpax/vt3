use reqwest::{multipart::Form, Client, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::VtResult;

/// Process a regular reqwest response
#[inline]
async fn process_resp<T>(resp: Response) -> VtResult<T>
where
    T: DeserializeOwned,
{
    let status = resp.status();

    match status {
        StatusCode::OK => Ok(resp.json().await?), // 200
        _ => Err((status, resp.text().await?).into()),
    }
}

/// Process a bzipped reqwest response
#[cfg(feature = "feeds")]
#[inline]
async fn process_resp_bz<T>(resp: Response) -> VtResult<Vec<T>>
where
    T: DeserializeOwned,
{
    use futures::stream::TryStreamExt;
    use tokio::io::AsyncReadExt;
    use tokio_util::io::StreamReader;

    fn convert_error(err: reqwest::Error) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::Other, err)
    }

    let status = resp.status();

    match status {
        StatusCode::OK => {
            let stream = resp.bytes_stream().map_err(convert_error);
            let stream_reader = StreamReader::new(stream);
            let mut decoder = async_compression::tokio::bufread::BzDecoder::new(stream_reader);

            let mut output = String::new();

            decoder.read_to_string(&mut output).await?;

            Ok(serde_json::from_str(&output)?) // 200
        }
        _ => Err((status, resp.text().await?).into()),
    }
}

/// GET from a URL
pub(crate) async fn http_get<T>(api_key: &str, user_agent: &str, url: &str) -> VtResult<T>
where
    T: DeserializeOwned,
{
    let client = Client::builder().user_agent(user_agent).build()?;
    let resp = client
        .get(url)
        .header("x-apikey", api_key)
        .header("Accept", "application/json")
        .send()
        .await?;
    process_resp(resp).await
}

/// GET from a URL with bzipped response
#[cfg(feature = "feeds")]
pub(crate) async fn http_get_bz<T>(api_key: &str, user_agent: &str, url: &str) -> VtResult<Vec<T>>
where
    T: DeserializeOwned,
{
    let client = Client::builder().user_agent(user_agent).build()?;
    let resp = client
        .get(url)
        .header("x-apikey", api_key)
        .header("Accept", "application/json")
        .send()
        .await?;
    process_resp_bz(resp).await
}

/// GET from a URL with query params
pub(crate) async fn http_get_with_params<T>(
    api_key: &str,
    user_agent: &str,
    url: &str,
    query_params: &[(&str, &str)],
) -> VtResult<T>
where
    T: DeserializeOwned,
{
    let client = Client::builder().user_agent(user_agent).build()?;
    let resp = client
        .get(url)
        .header("x-apikey", api_key)
        .header("Accept", "application/json")
        .query(query_params)
        .send()
        .await?;
    process_resp(resp).await
}

/// POST to a URL
pub(crate) async fn http_post<T>(
    api_key: &str,
    user_agent: &str,
    url: &str,
    form_data: &[(&str, &str)],
) -> VtResult<T>
where
    T: DeserializeOwned,
{
    let client = Client::builder().user_agent(user_agent).build()?;
    let resp = client
        .post(url)
        .header("x-apikey", api_key)
        .form(form_data)
        .send()
        .await?;
    process_resp(resp).await
}

/// POST to a URL with multipart form_data
pub(crate) async fn http_multipart_post<T>(
    api_key: &str,
    user_agent: &str,
    url: &str,
    form_data: Form,
) -> VtResult<T>
where
    T: DeserializeOwned,
{
    let client = Client::builder().user_agent(user_agent).build()?;
    let resp = client
        .post(url)
        .header("x-apikey", api_key)
        .multipart(form_data)
        .send()
        .await?;
    process_resp(resp).await
}

/// POST to a URL with data in the body
pub(crate) async fn http_body_post<S, T>(
    api_key: &str,
    user_agent: &str,
    url: &str,
    data: S,
) -> VtResult<T>
where
    S: Serialize,
    T: DeserializeOwned,
{
    let client = Client::builder().user_agent(user_agent).build()?;
    let resp = client
        .post(url)
        .header("x-apikey", api_key)
        .json(&data)
        .send()
        .await?;
    process_resp(resp).await
}

/// DELETE
pub(crate) async fn http_delete<T>(api_key: &str, user_agent: &str, url: &str) -> VtResult<T>
where
    T: DeserializeOwned,
{
    let client = Client::builder().user_agent(user_agent).build()?;
    let resp = client
        .delete(url)
        .header("x-apikey", api_key)
        .send()
        .await?;
    process_resp(resp).await
}

/// PATCH
#[cfg(feature = "hunting")]
pub(crate) async fn http_patch<S, T>(
    api_key: &str,
    user_agent: &str,
    url: &str,
    data: S,
) -> VtResult<T>
where
    S: Serialize,
    T: DeserializeOwned,
{
    let client = Client::builder().user_agent(user_agent).build()?;
    let resp = client
        .patch(url)
        .header("x-apikey", api_key)
        .json(&data)
        .send()
        .await?;
    process_resp(resp).await
}
