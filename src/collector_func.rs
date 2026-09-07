use anyhow::Result;
use k8s_openapi::jiff::Timestamp;
use k8s_openapi::{
    api::core::v1::{Namespace, Pod},
    apimachinery::pkg::apis::meta::v1::Time,
};
use kube::Client;

/////////////////////PODS/////////////////////
pub fn get_pod_name(pod: &Pod) -> &str {
    pod.metadata.name.as_deref().unwrap_or("unknown")
}

pub fn get_pod_namespace(pod: &Pod) -> &str {
    pod.metadata.namespace.as_deref().unwrap_or("unknown")
}

pub fn get_pod_restart_count(pod: &Pod) -> i32 {
    let mut restart_count = 0;
    if let Some(pod_status) = &pod.status {
        if let Some(container_status) = &pod_status.container_statuses {
            for cs in container_status {
                restart_count += cs.restart_count;
            }
        }
    }
    restart_count
}

pub fn get_pods_phase(pod: &Pod) -> Option<&str> {
    pod.status
        .as_ref()
        .and_then(|status| status.phase.as_deref())
}

/////////////////////NAMESPACE/////////////////////

pub fn get_namespace_name(namespace: &Namespace) -> &str {
    namespace.metadata.name.as_deref().unwrap_or("unknown")
}

pub fn get_namespace_status(namespace: &Namespace) -> Option<&str> {
    namespace
        .status
        .as_ref()
        .and_then(|status| status.phase.as_deref())
}

pub fn get_namespace_age(namespace: &Namespace) -> Option<Time> {
    // namespace.metadata.creation_timestamp.clone()
}

pub fn get_namespace_creation_time(namespace: &Namespace) -> Option<Timestamp> {
    namespace
        .metadata
        .creation_timestamp
        .as_ref()
        .map(|time| time.0)
}
/////////////////////CLIENT/////////////////////
pub async fn get_client() -> Result<Client> {
    let client = Client::try_default().await?;
    Ok(client)
}
