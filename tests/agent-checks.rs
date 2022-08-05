use consul_oxide::{AgentChecks, Client, Config, RegisterCheckPayload};

#[tokio::test]
async fn test_register_check() {
    let client = Client::new(Config::default());
    let check = RegisterCheckPayload { name: "test_check".to_string(), ..Default::default() };
    client.register_check(check).await.expect("failed to register check");

    let checks = client.list_checks().await.unwrap();
    assert!(checks.contains_key("test_check"));

    client.deregister_check("test_check").await.expect("failed to deregister check");
}
