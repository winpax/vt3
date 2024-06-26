use reqwest::multipart::{Form, Part};
use std::{fs::File, io::Read};

mod response;
use response::{Attributes, Root, ScanRoot};
mod model;

pub use model::VtFiles;

use crate::{
    utils::{http_get, http_multipart_post, http_post},
    VtClient, VtResult,
};

impl VtClient {
    pub async fn file_info(&self, id: &str) -> VtResult<Root> {
        //! Retrieve public_api.file scan reports
        //! id: SHA-256, SHA-1 or MD5 identifying the public_api.file
        //!
        //! ## Example Usage
        //! ```rust
        //! use async_vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! vt.file_info("44d88612fea8a8f36de82e1278abb02f");
        //! ```
        let url = format!("{}/files/{}", &self.endpoint, id);
        http_get(&self.api_key, &self.user_agent, &url).await
    }

    pub async fn file_scan(&self, file: &str) -> VtResult<ScanRoot> {
        //! Upload and scan a public_api.file
        //!
        //! ## Example Usage
        //! ```rust
        //! use async_vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! # tokio_test::block_on(async {
        //! println!("{:?}", vt.file_scan("data/eicar.com.txt").await);
        //! # })
        //! ```
        let mut f = File::open(file)?;
        let mut buffer = Vec::new();
        {
            f.read_to_end(&mut buffer)?;
        }
        let form_data = Form::new().part("file", Part::bytes(buffer).file_name(file.to_owned()));
        let url = format!("{}/files", &self.endpoint);
        http_multipart_post(&self.api_key, &self.user_agent, &url, form_data).await
    }

    pub async fn file_rescan(&self, id: &str) -> VtResult<ScanRoot> {
        //! Re-submit/Re-scan already submitted files
        //! id: SHA-256, SHA-1 or MD5 identifying the public_api.file
        //!
        //! ## Example Usage
        //! ```rust
        //! use async_vt3::VtClient;
        //!
        //! let vt = VtClient::new("Your API Key");
        //! vt.file_rescan("44d88612fea8a8f36de82e1278abb02f");
        //! ```
        let url = format!("{}/files/{}/analyse", &self.endpoint, id);
        let form_data = &[("id", id)];
        http_post(&self.api_key, &self.user_agent, &url, form_data).await
    }
}
