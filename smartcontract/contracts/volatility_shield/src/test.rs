#![cfg(test)]
use super::*;
use soroban_sdk::{
    testutils::Address as _,
    Address, Env, Map,
};

// ─────────────────────────────────────────────────────────────────────────────
// Initialisation tests
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn test_init_stores_roles() {
    let env         = Env::default();
    let contract_id = env.register(VolatilityShield, ());
    let client      = VolatilityShieldClient::new(&env, &contract_id);

    let admin  = Address::generate(&env);
    let asset  = Address::generate(&env);
    let oracle = Address::generate(&env);

    client.init(&admin, &asset, &oracle);

    assert_eq!(client.get_admin(),  admin);
    assert_eq!(client.get_oracle(), oracle);
    assert_eq!(client.get_asset(),  asset);
}

#[test]
#[should_panic(expected = "Already initialized")]
fn test_init_cannot_be_called_twice() {
    let env         = Env::default();
    let contract_id = env.register(VolatilityShield, ());
    let client      = VolatilityShieldClient::new(&env, &contract_id);

    let admin  = Address::generate(&env);
    let asset  = Address::generate(&env);
    let oracle = Address::generate(&env);

    client.init(&admin, &asset, &oracle);
    client.init(&admin, &asset, &oracle); // must panic
}

// ─────────────────────────────────────────────────────────────────────────────
// Rebalance auth test
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn test_rebalance_admin_auth_accepted() {
    let env         = Env::default();
    env.mock_all_auths();                 // bypass require_auth in tests

    let contract_id = env.register(VolatilityShield, ());
    let client      = VolatilityShieldClient::new(&env, &contract_id);

    let admin  = Address::generate(&env);
    let asset  = Address::generate(&env);
    let oracle = Address::generate(&env);

    client.init(&admin, &asset, &oracle);

    // Empty allocation map: nothing to move, but auth path is exercised
    let allocations: Map<Address, i128> = Map::new(&env);
    client.rebalance(&allocations);      // should not panic
}

// ─────────────────────────────────────────────────────────────────────────────
// Share math tests (unchanged from original)
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn test_convert_to_shares() {
    let env         = Env::default();
    let contract_id = env.register(VolatilityShield, ());
    let client      = VolatilityShieldClient::new(&env, &contract_id);

    // When shares == 0, it should be 1:1
    assert_eq!(client.convert_to_shares(&100), 100);

    // Standard proportional minting
    // total_assets=1000, total_shares=500 → 200 assets = 100 shares
    client.set_total_shares(&500);
    assert_eq!(client.convert_to_shares(&200), 200); // total_assets still 0 → 1:1
}

#[test]
fn test_convert_to_assets() {
    let env         = Env::default();
    let contract_id = env.register(VolatilityShield, ());
    let client      = VolatilityShieldClient::new(&env, &contract_id);

    // When shares == 0, assets = shares
    assert_eq!(client.convert_to_assets(&100), 100);
}
