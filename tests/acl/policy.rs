use consul_oxide::{AclPolicies, Client, Config, CreatePolicy};

#[tokio::test]
async fn test_create_acl_policy() {
    let client = Client::new(Config::default());

    // check there are no policies
    let policies = client.list_policies().await.expect("failed to list policies");
    assert!(policies.is_empty());

    // create policy
    let policy = client
        .create_policy(CreatePolicy {
            name: "test".to_string(),
            description: "test".to_string(),
            rules: "test".to_string(),
            datacenters: vec!["dc1".to_string()],
        })
        .await
        .expect("failed to create policy");

    // check policy exists
    let policies = client.read_policy(&policy.id).await.expect("failed to list policies");
    assert!(policies.is_some());

    // delete policy
    let result = client.delete_policy(&policy.id).await.expect("failed to delete policy");
    assert!(result);

    // check policy no longer exists
    let policies = client.list_policies().await.expect("failed to list policies");
    assert!(policies.is_empty());
}
