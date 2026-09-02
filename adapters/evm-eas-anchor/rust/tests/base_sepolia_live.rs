use acta_evm_eas_anchor::{
    EasRpc, HttpEasRpc, BASE_SEPOLIA_CHAIN_ID, BASE_SEPOLIA_EAS_ADDRESS,
    BASE_SEPOLIA_SCHEMA_REGISTRY_ADDRESS,
};

/// Read-only deployment preflight. It is ignored in the default suite because a passing unit
/// test must not depend on network availability.
#[test]
#[ignore = "requires public Base Sepolia JSON-RPC"]
fn official_base_sepolia_contracts_are_reachable() {
    let rpc = HttpEasRpc::new("https://sepolia.base.org").unwrap();
    assert_eq!(rpc.chain_id().unwrap(), BASE_SEPOLIA_CHAIN_ID);
    assert!(!rpc.code_at(BASE_SEPOLIA_EAS_ADDRESS).unwrap().is_empty());
    assert!(!rpc
        .code_at(BASE_SEPOLIA_SCHEMA_REGISTRY_ADDRESS)
        .unwrap()
        .is_empty());
}
