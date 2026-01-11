use std::{collections::HashMap, env, path::PathBuf, str::FromStr, sync::LazyLock};

use anyhow::Result;
use nimbuspulse_client::{DcsRuntime, Uuid};

use crate::{
    manifest::Manifest, manifests::truth_files_from_manifests, sync::compare_and_maybe_upload,
};

mod manifest;
mod manifest_file;
mod manifests;
mod sync;

static SAVE_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| PathBuf::from_str(&env::var("MANIFEST_DIR").unwrap()).unwrap());

pub(crate) type Runtimes = HashMap<Uuid, (DcsRuntime, String)>;

pub async fn execute() -> Result<()> {
    println!("[?] Using manifest directory: {:?}", *SAVE_DIR);

    let sync_path = env::var("SYNC_PATH")?;
    let client = nimbuspulse_client::Client::new(env::var("NIMBUSPULSE_API_KEY")?);
    let sync_instances_string = env::var("NIMBUSPULSE_SYNC_INSTANCE_IDS")?;
    let sync_instance_ids = sync_instances_string
        .split(",")
        .map(|id| id.trim())
        .map(|id| Uuid::from_str(id).unwrap())
        .collect::<Vec<Uuid>>();
    let mut manifests = Vec::new();
    let mut runtimes: Runtimes = HashMap::new();

    println!("[?] Syncing files in \"{}\" between:", sync_path);
    for instance_id in sync_instance_ids.iter() {
        let runtime = client.get_runtime(instance_id).await?;
        let name = runtime.clone().settings.unwrap().settings.name;
        runtimes.insert(*instance_id, (runtime.clone(), name.clone()));

        println!("    [?] {} ({})", name, instance_id);
    }

    for instance_id in sync_instance_ids.iter() {
        let (_runtime, name) = runtimes.get(&instance_id).unwrap();
        println!(
            "[+] Building manifest for instance {} ({})",
            name, instance_id
        );

        let files = client.list_files(&instance_id, &sync_path).await?.files;

        let mut manifest = Manifest::load_or_new(*instance_id, files).await;
        manifest.build(&client).await?;
        manifests.push(manifest);
    }

    compare_and_maybe_upload(
        truth_files_from_manifests(&manifests).await,
        &manifests,
        sync_instance_ids,
        &runtimes,
        &client,
    )
    .await;

    Ok(())
}
