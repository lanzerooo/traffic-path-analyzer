use anyhow::{Ok, Result};
use k8s_openapi::api::{
    apps::v1::Deployment,
    core::v1::{Namespace, Node, Pod},
};
use kube::{
    Api,
    api::{ListParams, ObjectList},
};

#[path = "collector_func.rs"]
mod collector_func;

pub struct ClusterSnapshot {
    // pub namespace: ObjectList<Namespace>,
    pub deployment: ObjectList<Deployment>,
    pub pod: ObjectList<Pod>,
}

pub async fn collect() -> Result<ClusterSnapshot> {
    // let namespaces = collect_namespace().await?;
    let deployments = collect_deploy().await?;
    let pods = collect_pods().await?;
    let cs = ClusterSnapshot {
        // namespace: namespaces,
        deployment: deployments,
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

async fn collect_namespace() -> Result<ObjectList<Namespace>> {
    let client = collector_func::get_client().await?;
    let namespaces: Api<Namespace> = Api::all(client);
    let lp = ListParams::default();
    let namespaces_list = namespaces.list(&lp).await?;
    Ok(namespaces_list)
}
