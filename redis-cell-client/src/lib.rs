mod command;

pub use command::{Cmd, Policy};

#[cfg(test)]
mod tests {
    use crate::{Cmd, Policy};
    use std::time::Duration;
    use testcontainers::core::IntoContainerPort as _;
    use testcontainers::runners::AsyncRunner;
    use testcontainers::{GenericImage, core::WaitFor};

    async fn it_works_with(image: &str) {
        let container = GenericImage::new(image, "latest")
            .with_exposed_port(6379.tcp())
            .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
            .start()
            .await
            .unwrap();
        let port = container.get_host_port_ipv4(6379).await.unwrap();
        let client = redis::Client::open(("localhost", port)).unwrap();
        let config = redis::aio::ConnectionManagerConfig::new().set_number_of_retries(1);
        let mut manager = redis::aio::ConnectionManager::new_with_config(client, config)
            .await
            .unwrap();

        let mut builder = Policy::builder();
        let policy = builder
            .burst(1usize)
            .tokens(10usize)
            .period(Duration::from_secs(60))
            .apply(1usize)
            .build();
        let cmd = Cmd::new("user123", &policy).into();
        let res = manager.send_packed_command(&cmd).await.unwrap();
        dbg!(res);
        let res = manager.send_packed_command(&cmd).await.unwrap();
        dbg!(res);
        let res = manager.send_packed_command(&cmd).await.unwrap();
        dbg!(res);
    }

    #[tokio::test]
    async fn it_works_with_redis() {
        it_works_with("redis-cell").await
    }

    #[tokio::test]
    async fn it_works_with_valkey() {
        it_works_with("valkey-cell").await
    }
}
