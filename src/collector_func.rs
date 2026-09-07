use anyhow::Result;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Pod;
use kube::Client;
use std::collections::BTreeMap;

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

pub fn get_pod_owner_references(pod: &Pod) -> Option<&str> {
    pod.metadata.owner_references.as_ref().and_then(|refs| {
        refs.iter()
            .find(|owner_ref| owner_ref.kind == "ReplicaSet")
            .map(|owner_ref| owner_ref.name.as_str())
    })
}

pub fn get_pod_image(pod: &Pod) -> Vec<&str> {
    pod.spec
        .as_ref()
        .into_iter()
        .flat_map(|spec| {
            spec.containers
                .iter()
                .chain(spec.init_containers.as_ref().into_iter().flatten())
        })
        .filter_map(|c| c.image.as_deref())
        .collect()
}

/////////////////////NAMESPACE/////////////////////

// pub fn get_namespace_name(namespace: &Namespace) -> &str {
//     namespace.metadata.name.as_deref().unwrap_or("unknown")
// }

// pub fn get_namespace_status(namespace: &Namespace) -> Option<&str> {
//     namespace
//         .status
//         .as_ref()
//         .and_then(|status| status.phase.as_deref())
// }

// pub fn get_namespace_age(namespace: &Namespace) -> Option<Time> {
//     // namespace.metadata.creation_timestamp.clone()
// }

// pub fn get_namespace_creation_time(namespace: &Namespace) -> Option<Timestamp> {
//     namespace
//         .metadata
//         .creation_timestamp
//         .as_ref()
//         .map(|time| time.0)
// }

/////////////////////DEPLOYMENT/////////////////////
pub fn get_deployment_name(deployment: &Deployment) -> &str {
    deployment.metadata.name.as_deref().unwrap_or("unknown")
}

pub fn get_deployment_namesapce(deployment: &Deployment) -> &str {
    deployment
        .metadata
        .namespace
        .as_deref()
        .unwrap_or("unknown")
}

pub fn get_deployment_replicas(deployment: &Deployment) -> Option<i32> {
    deployment.status.as_ref().and_then(|repl| repl.replicas)
}

pub fn get_deployment_available_replicas(deployment: &Deployment) -> Option<i32> {
    deployment
        .status
        .as_ref()
        .and_then(|available_replicas| available_replicas.available_replicas)
}

pub fn get_deployment_ready_replicas(deployment: &Deployment) -> Option<i32> {
    deployment
        .status
        .as_ref()
        .and_then(|ready_repl| ready_repl.ready_replicas)
}

pub fn get_deployment_selector(deployment: &Deployment) -> Option<&BTreeMap<String, String>> {
    deployment
        .spec
        .as_ref()
        .and_then(|spec| spec.selector.match_labels.as_ref())
}

/////////////////////CLIENT/////////////////////
pub async fn get_client() -> Result<Client> {
    let client = Client::try_default().await?;
    Ok(client)
}
