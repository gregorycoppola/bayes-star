// use bayes_star::model::storage::Graph;
// use bayes_star::model::objects::Entity;
// use bayes_star::model::objects::Domain;
// use std::sync::Arc;
// use redis::Client;

// #[test]
// fn test_store_entity() {
//     // Setup Redis client
//     let client = Arc::new(Client::open("redis://127.0.0.1/").unwrap());
//     let storage = Graph::new(client);

//     // Create an entity
//     let entity = Entity {
//         domain: Domain::Jack,
//         name: "test_entity".to_string(),
//     };

//     // Store the entity
//     storage.store_entity(&entity).unwrap();

//     // Assertions can be added here
//     // Note: Actual Redis interactions would require a running Redis server
// }

// #[test]
// fn test_get_entities_in_domain() {
//     // Setup Redis client
//     let client = Arc::new(Client::open("redis://127.0.0.1/").unwrap());
//     let storage = Graph::new(client);

//     // Attempt to retrieve entities from a domain
//     let entities = storage.get_entities_in_domain("test_domain").unwrap();

//     // Example assertion
//     assert!(entities.contains(&Entity {
//         domain: "test_domain".to_string(),
//         name: "test_entity".to_string(),
//     }));

//     // More assertions and cleanup can be added here
// }
