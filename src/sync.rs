use std::path::PathBuf;

use nimbuspulse_client::{Client, Uuid};

use crate::{Runtimes, manifest::Manifest, manifests::TruthFiles};

pub async fn compare_and_maybe_upload(
    truth_files: TruthFiles,
    manifests: &Vec<Manifest>,
    sync_instance_ids: Vec<Uuid>,
    runtimes: &Runtimes,
    client: &Client,
) {
    for (path, (instance_id, file, occurrences)) in truth_files {
        if occurrences == manifests.len() {
            continue;
        }

        let upload_to: Vec<Uuid> = sync_instance_ids
            .iter()
            .cloned()
            .filter(|id| *id != instance_id)
            .collect();

        println!("[+] Syncing file {} (O:{})", path, occurrences);

        for instance_id in upload_to {
            let (_runtime, name) = runtimes.get(&instance_id).unwrap();
            println!("[+] Uploading to instance {} ({})", name, instance_id);

            let pathbuf = PathBuf::from(path.clone());
            let dir = pathbuf.parent().unwrap();
            if let Err(err) = client
                .create_directory(&instance_id, dir.to_str().unwrap())
                .await
            {
                eprintln!("[!] Failed to create directory {:?}: {}", dir, err);
            }

            if let Err(err) = client
                .upload_file(&instance_id, path.clone(), file.content.clone())
                .await
            {
                eprintln!("[!] Failed to sync file {}: {}", path, err);
            }
        }
    }
}
