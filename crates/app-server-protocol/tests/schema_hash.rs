use ash_app_server_protocol::protocol::initialize::ProtocolVersion;
use ash_app_server_protocol::schema_hash;

#[test]
fn compiled_protocol_matches_generated_metadata() {
    let metadata: serde_json::Value =
        serde_json::from_str(include_str!("../../../.build/protocol/metadata.json")).unwrap();
    let version = ProtocolVersion::current();
    assert_eq!(
        metadata,
        serde_json::json!({
            "major": version.major,
            "schemaHash": schema_hash(),
        })
    );
}

#[cfg(feature = "export")]
#[test]
fn prepared_contract_matches_the_rust_generators() {
    use ash_app_server_protocol::json_schema;
    use ash_app_server_protocol::protocol_metadata;
    use std::path::Path;

    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.build/protocol");
    assert_eq!(
        std::fs::read_to_string(directory.join("json/schema.json"))
            .unwrap()
            .replace("\r\n", "\n"),
        json_schema()
    );
    assert_eq!(
        std::fs::read_to_string(directory.join("metadata.json"))
            .unwrap()
            .replace("\r\n", "\n"),
        protocol_metadata()
    );
}
