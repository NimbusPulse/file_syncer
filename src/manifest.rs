use anyhow::{Result, bail};
use nimbuspulse_client::{FileInfo, Uuid};
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::{SAVE_DIR, manifest_file::ManifestFile};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub instance_id: Uuid,
    files: Vec<FileInfo>,
    pub manifest_files: Vec<ManifestFile>,
}

impl Manifest {
    pub async fn load_or_new(instance_id: Uuid, files: Vec<FileInfo>) -> Self {
        match Self::try_load(instance_id, files.clone()).await {
            Ok(manifest) => manifest,
            Err(_) => Manifest {
                instance_id,
                files,
                manifest_files: Vec::new(),
            },
        }
    }

    pub async fn try_load(instance_id: Uuid, files: Vec<FileInfo>) -> Result<Self> {
        let manifest_file = SAVE_DIR.join(format!("{}.json", instance_id));

        if !manifest_file.exists() {
            bail!("Manifest not found in {:?}", manifest_file);
        }

        let content = fs::read_to_string(&manifest_file).await?;
        let manifest_files: Vec<ManifestFile> = serde_json::from_str(&content)?;

        Ok(Manifest {
            instance_id,
            files,
            manifest_files,
        })
    }

    pub async fn save(&self) -> Result<()> {
        let manifest_file = SAVE_DIR.join(format!("{}.json", self.instance_id));
        let content = serde_json::to_string(&self.manifest_files)?;

        fs::write(&manifest_file, content).await?;

        Ok(())
    }

    pub async fn build(&mut self, client: &nimbuspulse_client::Client) -> Result<()> {
        let manifest_file = SAVE_DIR.join(format!("{}.json", self.instance_id));
        fs::create_dir_all(&manifest_file.parent().unwrap()).await?;

        for file in self.files.iter() {
            if file.is_directory {
                continue;
            }

            if let Some(mf) = self
                .manifest_files
                .iter_mut()
                .find(|mf| mf.remote_path == file.path)
            {
                if mf.modified_date.unwrap_or(0) < file.modified.unwrap_or(0)
                    && mf.size.unwrap_or(0) != file.size.unwrap_or(0)
                {
                    mf.hydrate(client, &self.instance_id, &file).await?;
                }

                continue;
            }

            let mut manifest_file = ManifestFile::new(file.path.clone());
            manifest_file
                .hydrate(client, &self.instance_id, &file)
                .await?;
            self.manifest_files.push(manifest_file);
        }

        self.save().await?;

        Ok(())
    }
}
