mod collector;
mod collector_func;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let snapshot = collector::collect().await?;
    for n in &snapshot.namespace.items {
        println!(
            "NAME: {}\t STATUS: {}\n",
            collector_func::get_namespace_name(n),
            collector_func::get_namespace_status(n).unwrap_or("unknown"),
        );
    }

    for p in &snapshot.pod.items {
        println!(
            "NAME: {}\t NAMESPACE: {}\t PHASE: {}\t RESTARTS: {}\n",
            collector_func::get_pod_name(p),
            collector_func::get_pod_namespace(p),
            collector_func::get_pods_phase(p).unwrap_or("unknown"),
            collector_func::get_pod_restart_count(p)
        );
    }
    println!("Кол-во подов: {}", snapshot.pod.items.len());
    Ok(())
}
