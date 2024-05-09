use std::fmt::{Display, Formatter};

use crate::public_api::comment::Comments;
use crate::public_api::file::VtFiles;
use serde::{Deserialize, Serialize};
use serde_json::{to_value, Value};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Relationships {
    Comments,
    CommunicatingFiles,
    DownloadedFiles,
    // Graphs,
    // HistorialSslCertificates,
    // HistoricalWhoIs,
    RelatedComments,
    ReferrerFiles,
    // Resolutions,
    // Urls,
}

impl Display for Relationships {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match to_value(self).unwrap() {
            Value::String(val) => write!(f, "{val}"),
            _ => write!(f, ""),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RelatedObjects {
    Comments(Comments),
    Files(VtFiles),
}
