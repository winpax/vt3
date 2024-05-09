mod response;
use response::Root;

use crate::{utils::http_get, VtClient, VtResult};

impl VtClient {
    pub async fn domain_info(&self, domain: &str) -> VtResult<Root> {
        //! Get the report of a given Domain
        //!
        //! ## Example Usage
        //! ```rust
        //! use async_vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! # tokio_test::block_on(async {
        //! println!("{:?}", vt.domain_info("google.com").await)
        //! # })
        //! ```
        let url = format!("{}/domains/{}", &self.endpoint, domain);
        http_get(&self.api_key, &self.user_agent, &url).await
    }
}
