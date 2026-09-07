mod collector;
mod collector_func;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let snapshot = collector::collect().await?;
    println!("=============");
    println!("DEPLOYMENTS: ");
    println!("=============");
    for d in &snapshot.deployment.items {
        println!(
            "NAME: {}\n NAMESPACE: {}\n REPLICAS: {}\n AVAILABLE REPLICAS: {}\n READY REPLICAS: {}\n SELECTOR: {}",
            collector_func::get_deployment_name(d),
            collector_func::get_deployment_namespace(d),
            collector_func::get_deployment_replicas(d).unwrap_or(0),
            collector_func::get_deployment_available_replicas(d).unwrap_or(0),
            collector_func::get_deployment_ready_replicas(d).unwrap_or(0),
            collector_func::get_deployment_selector(d)
                .map(|labels| labels
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<_>>()
                    .join(", "))
                .unwrap_or_else(|| "none".to_string())
        )
    }
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("=============");
    println!("REPLICASETS: ");
    println!("=============");

    println!("=============");
    println!("PODS: ");
    println!("=============");
    for p in &snapshot.pod.items {
        println!(
            "NAME: {}\n NAMESPACE: {}\n PHASE: {}\n RESTARTS: {}\n POD OWNER: {}\n, IMAGE: {}\n",
            collector_func::get_pod_name(p),
            collector_func::get_pod_namespace(p),
            collector_func::get_pods_phase(p).unwrap_or("unknown"),
            collector_func::get_pod_restart_count(p),
            collector_func::get_pod_owner_references(p).unwrap_or("unknown"),
            if collector_func::get_pod_image(p).is_empty() {
                "unknown".to_string()
            } else {
                collector_func::get_pod_image(p).join(", ")
            }
        );
    }
    println!("Кол-во подов: {}", snapshot.pod.items.len());
    Ok(())
}
