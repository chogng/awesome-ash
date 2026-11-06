use super::{configure_browser_paths, configure_node_runtime};
use serde_json::json;
use std::collections::BTreeMap;

#[test]
fn vite_filesystem_sources_have_a_default_mapping_and_explicit_mappings_are_preserved() {
    let mut properties = BTreeMap::new();
    configure_browser_paths(&mut properties);
    assert_eq!(
        properties["pathMapping"]["/@fs/"],
        if cfg!(windows) { "" } else { "/" }
    );
    let mut properties =
        BTreeMap::from([("pathMapping".into(), json!({"/custom/":"/workspace/"}))]);
    let original = properties.clone();
    configure_browser_paths(&mut properties);
    assert_eq!(properties, original);
}

#[test]
fn managed_node_is_the_default_and_preserves_explicit_runtime_selection() {
    let node = std::path::Path::new("/managed/node");
    let mut properties = BTreeMap::new();
    configure_node_runtime(&mut properties, node);
    assert_eq!(properties["runtimeExecutable"], json!("/managed/node"));
    for property in ["runtimeExecutable", "runtimeVersion"] {
        let mut properties = BTreeMap::from([(property.into(), json!("user-selected"))]);
        let original = properties.clone();
        configure_node_runtime(&mut properties, node);
        assert_eq!(properties, original);
    }
}
