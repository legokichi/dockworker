use dockworker::{container::ContainerStdio, ContainerCreateOptions, ContainerHostConfig, Docker};

#[tokio::main]
async fn main() {
    let docker = Docker::connect_with_defaults().unwrap();
    let mut host_config = ContainerHostConfig::new();
    host_config.auto_remove(true);
    let mut create = ContainerCreateOptions::new("hello-world:linux");
    create.host_config(host_config);

    let container = docker
        .create_container(Some("testing"), &create)
        .await
        .unwrap();
    docker.start_container(&container.id).await.unwrap();
    let mut res = docker
        .attach_container(&container.id, None, true, true, false, true, false)
        .await
        .unwrap();

    use futures::stream::StreamExt;
    while let Some(stdio) = res.next().await {
        match stdio {
            ContainerStdio::Stdin(buf) => {
                println!("{}", String::from_utf8(buf).unwrap());
            }
            ContainerStdio::Stdout(buf) => {
                println!("{}", String::from_utf8(buf).unwrap());
            }
            ContainerStdio::Stderr(buf) => {
                println!("{}", String::from_utf8(buf).unwrap());
            }
        }
    }
}
