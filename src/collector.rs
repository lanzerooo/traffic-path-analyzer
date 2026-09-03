use anyhow::Result;
use k8s_openapi::api::core::v1::{Node, Pod};
use kube::{
    Api,
    api::{ListParams, ObjectList},
};

#[path = "collector_func.rs"]
mod collector_func;

pub struct ClusterSnapshot {
    pub pod: ObjectList<Pod>,
}

pub async fn collect() -> Result<ClusterSnapshot> {
    let pods = collect_pods().await?;
    let cs_pods = ClusterSnapshot { pod: pods };
    Ok(cs_pods)
}

async fn collect_pods() -> Result<ObjectList<Pod>> {
    let client = collector_func::get_client().await?;
    let pods: Api<Pod> = Api::namespaced(client, "istio-system");
    let lp = ListParams::default();
    let pod_list = pods.list(&lp).await?;
    Ok(pod_list)
}

// async fn collect_nodes() -> Result<ObjectList<Node>> {
//     let client = collector_func::get_client().await?;
// }
