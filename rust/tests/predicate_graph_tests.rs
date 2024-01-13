#[test]
fn test_store_entity() {
    // Setup Redis client
    let client = Arc::new(Client::open("redis://127.0.0.1/").unwrap());
    let storage = GraphicalModel::new(client);

    // Create an entity
    let entity = Entity {
        domain: Domain::Jack,
        name: "test_entity".to_string(),
    };

    // Store the entity
    storage.store_entity(&entity).unwrap();

    // Assertions can be added here
    // Note: Actual Redis interactions would require a running Redis server
}