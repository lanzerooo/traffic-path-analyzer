use anyhow::Result;
use k8s_openapi::api::apps::v1::{Deployment, ReplicaSet};
use k8s_openapi::api::core::v1::ServicePort;
use k8s_openapi::api::core::v1::{Pod, Service};
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

/////////////////////DEPLOYMENT/////////////////////
pub fn get_deployment_name(deployment: &Deployment) -> &str {
    deployment.metadata.name.as_deref().unwrap_or("unknown")
}

pub fn get_deployment_namespace(deployment: &Deployment) -> &str {
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

/////////////////////REPLICASETS/////////////////////
pub fn get_replicaset_name(replicaset: &ReplicaSet) -> &str {
    replicaset.metadata.name.as_deref().unwrap_or("unknown")
}

pub fn get_replicaset_namespace(replicaset: &ReplicaSet) -> &str {
    replicaset
        .metadata
        .namespace
        .as_deref()
        .unwrap_or("unknown")
}

pub fn get_replicaset_desired_replicas(replicaset: &ReplicaSet) -> Option<i32> {
    replicaset.spec.as_ref().and_then(|spec| spec.replicas)
}

pub fn get_replicaset_available_replicas(replicaset: &ReplicaSet) -> Option<i32> {
    replicaset
        .status
        .as_ref()
        .and_then(|status| status.available_replicas)
}

pub fn get_replicaset_ready_replicas(replicaset: &ReplicaSet) -> Option<i32> {
    replicaset
        .status
        .as_ref()
        .and_then(|status| status.ready_replicas)
}

pub fn get_replicaset_selector(replicaset: &ReplicaSet) -> Option<&BTreeMap<String, String>> {
    replicaset
        .spec
        .as_ref()
        .and_then(|spec| spec.selector.match_labels.as_ref())
}

/////////////////////SERVICES/////////////////////

pub fn get_service_name(service: &Service) -> &str {
    service.metadata.name.as_deref().unwrap_or("unknown")
}

pub fn get_service_namespace(service: &Service) -> &str {
    service.metadata.namespace.as_deref().unwrap_or("unknown")
}

pub fn get_service_type(service: &Service) -> Option<&str> {
    service.spec.as_ref().and_then(|spec| spec.type_.as_deref())
}

pub fn get_service_cluster_ip(service: &Service) -> Option<&str> {
    service
        .spec
        .as_ref()
        .and_then(|spec| spec.cluster_ip.as_deref())
}

pub fn get_service_ports(service: &Service) -> Option<&[ServicePort]> {
    service.spec.as_ref().and_then(|spec| spec.ports.as_deref())
}

pub fn get_service_selector(service: &Service) -> Option<&BTreeMap<String, String>> {
    service
        .spec
        .as_ref()
        .and_then(|spec| spec.selector.as_ref())
}

/////////////////////CLIENT/////////////////////
pub async fn get_client() -> Result<Client> {
    let client = Client::try_default().await?;
    Ok(client)
}
