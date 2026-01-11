use anyhow::Result;
use blake2::{Blake2b512, Digest};
use nimbuspulse_client::{FileInfo, Uuid};
use serde::{Deserialize, Serialize};
use urlencoding::encode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestFile {
    pub remote_path: String,
    pub modified_date: Option<u64>,
    pub size: Option<u64>,
    pub content: Vec<u8>,
    pub hash: Option<String>,
}

impl ManifestFile {
    pub fn new(remote_path: String) -> Self {
        ManifestFile {
            remote_path,
            modified_date: None,
            size: None,
            content: vec![],
            hash: None,
        }
    }

    pub async fn hydrate(
        &mut self,
        client: &nimbuspulse_client::Client,
        instance_id: &Uuid,
        remote_file: &FileInfo,
    ) -> Result<()> {
        println!(
            "[+] Downloading file {} (Now {}, was {})",
            self.remote_path,
            remote_file.modified.unwrap_or(0),
            self.modified_date.unwrap_or(0)
        );

        let content = client
            .download_file(instance_id, encode(&self.remote_path))
            .await?;

        self.content = content;
        self.modified_date = remote_file.modified;
        self.size = remote_file.size;

        let mut hasher = Blake2b512::new();
        hasher.update(&self.content);
        self.hash = Some(format!("{:x}", hasher.finalize()));

        Ok(())
    }
}
