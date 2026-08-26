use anyhow::Result;
use k8s_openapi::api::core::v1::Pod;
use kube::{api::ListParams, Api, Client};

#[tokio::main]
async fn main() -> Result<()> {
    //создание клиента k8s
    let client = Client::try_default().await?;

    //создание интерфейса для работы с подами в неймпесей mobile-apps
    let pods: Api<Pod> = Api::namespaced(client, "mobile-apps");

    //получение списка всех подов
    let lp = ListParams::default();
    let pod_list = pods.list(&lp).await?;

    println!("{:<30} {:<15} {:<10}", "POD NAME", "STATUS", "RESTARTS");
    println!("{}", "-".repeat(60));

    for p in pod_list {
        let name = p.metadata.name.unwrap_or_else(|| "unknown".to_string());

        let status = p.status.as_ref().and_then(|s| s.phase.clone()).unwrap_or_else(|| "unknown".to_string());

        let mut restart_count = 0;
        if let Some(status_obj) = &p.status {
            if let Some(container_statuses) = &status_obj.container_statuses {
                for cs in container_statuses {
                    restart_count += cs.restart_count;
                }
            }
        }
        println!("{:<30} {:<15} {:<10}", name, status, restart_count);
    }
    Ok(())
}