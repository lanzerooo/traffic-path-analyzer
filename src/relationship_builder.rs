use k8s_openapi::api::apps::v1::ReplicaSet;
use k8s_openapi::api::core::v1::{Pod, ServicePort};
use std::collections::BTreeMap;

use crate::collector;

use crate::collector_func;

pub struct DeploymentNode {
    pub name: String,
    pub namespace: String,

    pub desired_replicas: Option<i32>,
    pub current_replicas: Option<i32>,
    pub available_replicas: Option<i32>,
    pub ready_replicas: Option<i32>,

    pub selector: Option<BTreeMap<String, String>>,

    pub replicasets: Vec<ReplicaSetNode>,
}

pub struct ReplicaSetNode {
    pub name: String,
    pub namespace: String,

    pub desired_replicas: Option<i32>,
    pub available_replicas: Option<i32>,
    pub ready_replicas: Option<i32>,

    pub selector: Option<BTreeMap<String, String>>,

    pub pods: Vec<PodNode>,
}

pub struct PodNode {
    pub name: String,
    pub namespace: String,

    pub phase: Option<String>,
    pub restart_count: i32,

    pub labels: BTreeMap<String, String>,

    pub services: Vec<ServiceNode>,
}

pub struct ServiceNode {
    pub name: String,
    pub namespace: String,

    pub service_type: Option<String>,
    pub cluster_ip: Option<String>,
    pub ports: Vec<ServicePort>,
    pub selector: Option<BTreeMap<String, String>>,
}

pub struct RelationshipResult {
    pub deployments: Vec<DeploymentNode>,

    pub orphan_replicasets: Vec<ReplicaSetNode>,
    pub orphan_pods: Vec<PodNode>,
}

pub fn get_deployment(deploy: &collector::ClusterSnapshot) -> Vec<DeploymentNode> {
    deploy
        .deployment
        .items
        .iter()
        .map(|deployment| DeploymentNode {
            name: collector_func::get_deployment_name(deployment).to_string(),
            namespace: collector_func::get_deployment_namespace(deployment).to_string(),
            desired_replicas: collector_func::get_deployment_desired_replicas(deployment),
            current_replicas: collector_func::get_deployment_current_replicas(deployment),
            available_replicas: collector_func::get_deployment_available_replicas(deployment),
            ready_replicas: collector_func::get_deployment_ready_replicas(deployment),
            selector: collector_func::get_deployment_selector(deployment).cloned(),
            replicasets: deploy
                .replicaset
                .items
                .iter()
                .filter(|replicasets| {
                    collector_func::get_replicaset_owner_references(replicasets)
                        == Some(collector_func::get_deployment_name(deployment))
                        && collector_func::get_replicaset_namespace(replicasets)
                            == collector_func::get_deployment_namespace(deployment)
                })
                .map(|replicaset| build_replicaset_node(replicaset, deploy))
                .collect(),
        })
        .collect()
}

pub fn get_replicas(snapshot: &collector::ClusterSnapshot) -> Vec<ReplicaSetNode> {
    snapshot
        .replicaset
        .items
        .iter()
        .map(|replicaset| build_replicaset_node(replicaset, snapshot))
        .collect()
}

fn pod_to_node(pod: &Pod) -> PodNode {
    PodNode {
        name: collector_func::get_pod_name(pod).to_string(),
        namespace: collector_func::get_pod_namespace(pod).to_string(),
        phase: collector_func::get_pods_phase(pod).map(|p| p.to_string()),
        restart_count: collector_func::get_pod_restart_count(pod),
        labels: collector_func::get_pod_labels(pod)
            .cloned()
            .unwrap_or_default(),
        services: Vec::new(),
    }
}

fn build_replicaset_node(
    replicaset: &ReplicaSet,
    snapshot: &collector::ClusterSnapshot,
) -> ReplicaSetNode {
    ReplicaSetNode {
        name: collector_func::get_replicaset_name(replicaset).to_string(),
        namespace: collector_func::get_replicaset_namespace(replicaset).to_string(),
        desired_replicas: collector_func::get_replicaset_desired_replicas(replicaset),
        available_replicas: collector_func::get_replicaset_available_replicas(replicaset),
        ready_replicas: collector_func::get_replicaset_ready_replicas(replicaset),
        selector: collector_func::get_replicaset_selector(replicaset).cloned(),
        pods: snapshot
            .pod
            .items
            .iter()
            .filter(|pods| {
                collector_func::get_pod_owner_references(pods)
                    == Some(collector_func::get_replicaset_name(replicaset))
                    && collector_func::get_pod_namespace(pods)
                        == collector_func::get_replicaset_namespace(replicaset)
            })
            .map(pod_to_node)
            .collect(),
    }
}
