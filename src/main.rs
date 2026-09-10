mod collector;
mod collector_func;
mod relationship_builder;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let snapshot = collector::collect().await?;
    let deployments = relationship_builder::get_deployment(&snapshot);
    println!("=============");
    println!("RESULT: ");
    println!("=============");
    for deployment in &deployments {
        println!("Deployment: {}/{}", deployment.namespace, deployment.name);

        for replicaset in &deployment.replicasets {
            println!("  ReplicaSet: {}", replicaset.name);

            for pod in &replicaset.pods {
                println!("    Pod: {}", pod.name);
            }
        }
    }
    // println!("=============");
    // println!("DEPLOYMENTS: ");
    // println!("=============");
    // for d in &snapshot.deployment.items {
    //     println!(
    //         "NAME: {}\n NAMESPACE: {}\n REPLICAS: {}\n AVAILABLE REPLICAS: {}\n READY REPLICAS: {}\n SELECTOR: {}",
    //         collector_func::get_deployment_name(d),
    //         collector_func::get_deployment_namespace(d),
    //         collector_func::get_deployment_current_replicas(d).unwrap_or(0),
    //         collector_func::get_deployment_available_replicas(d).unwrap_or(0),
    //         collector_func::get_deployment_ready_replicas(d).unwrap_or(0),
    //         collector_func::get_deployment_selector(d)
    //             .map(|labels| labels
    //                 .iter()
    //                 .map(|(k, v)| format!("{}={}", k, v))
    //                 .collect::<Vec<_>>()
    //                 .join(", "))
    //             .unwrap_or_else(|| "none".to_string())
    //     )
    // }
    // println!("Кол-во деплойментов: {}\n", snapshot.deployment.items.len());
    // println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    // println!("=============");
    // println!("REPLICASETS: ");
    // println!("=============");
    // for r in &snapshot.replicaset.items {
    //     println!(
    //         "NAME: {}\n NAMESPACE: {}\n REPLICASET OWNER: {}\n DESIRED REPLICAS: {}\n AVAILABLE REPLICAS: {}\n READY REPLICAS: {}\n SELECTOR: {}",
    //         collector_func::get_replicaset_name(r),
    //         collector_func::get_replicaset_namespace(r),
    //         collector_func::get_replicaset_owner_references(r).unwrap_or("unknown"),
    //         collector_func::get_replicaset_desired_replicas(r).unwrap_or(0),
    //         collector_func::get_replicaset_available_replicas(r).unwrap_or(0),
    //         collector_func::get_replicaset_ready_replicas(r).unwrap_or(0),
    //         collector_func::get_replicaset_selector(r)
    //             .map(|labels| labels
    //                 .iter()
    //                 .map(|(k, v)| format!("{}={}", k, v))
    //                 .collect::<Vec<_>>()
    //                 .join(", "))
    //             .unwrap_or_else(|| "none".to_string())
    //     )
    // }
    // println!(
    //     "\nКол-во репликасетов: {}\n",
    //     snapshot.replicaset.items.len()
    // );
    // println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    // println!("=============");
    // println!("PODS: ");
    // println!("=============");
    // for p in &snapshot.pod.items {
    //     println!(
    //         "NAME: {}\n NAMESPACE: {}\n POD OWNER: {}\n PHASE: {}\n RESTARTS: {}\n IMAGE: {}\n",
    //         collector_func::get_pod_name(p),
    //         collector_func::get_pod_namespace(p),
    //         collector_func::get_pod_owner_references(p).unwrap_or("unknown"),
    //         collector_func::get_pods_phase(p).unwrap_or("unknown"),
    //         collector_func::get_pod_restart_count(p),
    //         if collector_func::get_pod_image(p).is_empty() {
    //             "unknown".to_string()
    //         } else {
    //             collector_func::get_pod_image(p).join(", ")
    //         }
    //     );
    // }
    // println!("\nКол-во подов: {}\n", snapshot.pod.items.len());
    // println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    // println!("=============");
    // println!("SERVICES: ");
    // println!("=============");
    // for s in &snapshot.service.items {
    //     println!(
    //         "NAME: {}\n NAMESPACE: {}\n TYPE: {}\n CLUSTER IP: {}\n PORTS: {}\n SELECTOR: {}\n",
    //         collector_func::get_service_name(s),
    //         collector_func::get_service_namespace(s),
    //         collector_func::get_service_type(s).unwrap_or("unknown"),
    //         collector_func::get_service_cluster_ip(s).unwrap_or("unknown"),
    //         collector_func::get_service_ports(s)
    //             .map(|ports| ports
    //                 .iter()
    //                 .map(|p| format!("{}/{}", p.port, p.protocol.as_deref().unwrap_or("TCP")))
    //                 .collect::<Vec<_>>()
    //                 .join(", "))
    //             .unwrap_or_else(|| "none".to_string()),
    //         collector_func::get_service_selector(s)
    //             .map(|selector| selector
    //                 .iter()
    //                 .map(|(k, v)| format!("{}={}", k, v))
    //                 .collect::<Vec<_>>()
    //                 .join(", "))
    //             .unwrap_or_else(|| "none".to_string()),
    //     )
    // }
    // println!("\nКол-во сервисов: {}\n", snapshot.service.items.len());
    Ok(())
}
