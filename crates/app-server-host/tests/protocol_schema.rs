use ash_app_server_protocol::schema_hash;

#[test]
fn product_dependency_graph_preserves_the_generated_protocol_schema_hash() {
    let metadata: serde_json::Value =
        serde_json::from_str(include_str!("../../../.build/protocol/metadata.json")).unwrap();
    assert_eq!(metadata["schemaHash"], schema_hash());
}
