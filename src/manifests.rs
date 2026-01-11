use std::collections::HashMap;

use nimbuspulse_client::Uuid;

use crate::{manifest::Manifest, manifest_file::ManifestFile};

/// InstanceId, ManifestFile, Occurrences
pub(crate) type TruthFiles = HashMap<String, (Uuid, ManifestFile, usize)>;

pub async fn truth_files_from_manifests(manifests: &Vec<Manifest>) -> TruthFiles {
    let mut truth_files: TruthFiles = HashMap::new();

    for manifest in manifests.iter() {
        for mf in manifest.manifest_files.iter() {
            match truth_files.get(&mf.remote_path) {
                Some((existing_instance_id, existing, occurrences)) => {
                    // Same file
                    if existing.hash == mf.hash {
                        truth_files.insert(
                            mf.remote_path.clone(),
                            (*existing_instance_id, existing.clone(), occurrences + 1),
                        );
                        continue;
                    }

                    // Different file
                    if existing.modified_date < mf.modified_date {
                        truth_files.insert(
                            mf.remote_path.clone(),
                            (manifest.instance_id, mf.clone(), *occurrences),
                        );
                    }
                }
                None => {
                    truth_files.insert(
                        mf.remote_path.clone(),
                        (manifest.instance_id, mf.clone(), 1),
                    );
                }
            }
        }
    }

    truth_files
}
