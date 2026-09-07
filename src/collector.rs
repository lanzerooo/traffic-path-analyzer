use anyhow::{Ok, Result};
use k8s_openapi::api::{
    apps::v1::{Deployment, ReplicaSet},
    core::v1::Pod,
};
use kube::{
    Api,
    api::{ListParams, ObjectList},
};

#[path = "collector_func.rs"]
mod collector_func;

pub struct ClusterSnapshot {
    pub deployment: ObjectList<Deployment>,
    pub replicaset: ObjectList<ReplicaSet>,
    pub pod: ObjectList<Pod>,
}

pub async fn collect() -> Result<ClusterSnapshot> {
    let deployments = collect_deploy().await?;
    let replicasets = collect_replicaset().await?;
    let pods = collect_pods().await?;
    let cs = ClusterSnapshot {
        deployment: deployments,
        replicaset: replicasets,
        pod: pods,
    };
    Ok(cs)
}

async fn collect_pods() -> Result<ObjectList<Pod>> {
    let client = collector_func::get_client().await?;
    let pods: Api<Pod> = Api::all(client);
    let lp = ListParams::default();
    let pod_list = pods.list(&lp).await?;
    Ok(pod_list)
}

async fn collect_deploy() -> Result<ObjectList<Deployment>> {
    let client = collector_func::get_client().await?;
    let deployments: Api<Deployment> = Api::all(client);
    let lp = ListParams::default();
    let deployments_list = deployments.list(&lp).await?;
    Ok(deployments_list)
}

async fn collect_replicaset() -> Result<ObjectList<ReplicaSet>> {
    let client = collector_func::get_client().await?;
    let replicasets: Api<ReplicaSet> = Api::all(client);
    let lp = ListParams::default();
    let replicasets_list = replicasets.list(&lp).await?;
    Ok(replicasets_list)
}
