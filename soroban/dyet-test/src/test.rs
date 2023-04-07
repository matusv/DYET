#![cfg(test)]

use super::{Contract, ContractClient};
use soroban_sdk::{symbol, vec, Env};

#[test]
fn test_init_and_coverage() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let num_fns: u32 = 3;
    let features = vec![&env, 4, 6, 0, 8, 3, 7];
    let (_, _, coverage) = client.init(&num_fns, &features);
    assert_eq!(coverage, vec![&env, 0, 0, 0, 0, 0, 0]);

    let coverage_result = client.coverage();
    assert_eq!(coverage_result, vec![&env, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_max_coverage() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let num_fns: u32 = 3;
    let features = vec![&env, 4, 6, 0, 8, 3, 7];
    client.init(&num_fns, &features);

    let calls = [
        (0, 4),
        (0, 6),
        (1, 0),
        (1, 8),
        (2, 3),
        (2, 7),
    ];

    for &(fn_i, arg) in calls.iter() {
        client.call(&fn_i, &arg);
    }

    let coverage_result = client.coverage();
    assert_eq!(coverage_result, vec![&env, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_num_fns_1() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let num_fns: u32 = 1;
    let features = vec![&env, 2, 5];
    client.init(&num_fns, &features);

    let calls = [(0, 2), (0, 5)];

    for &(fn_i, arg) in calls.iter() {
        client.call(&fn_i, &arg);
    }

    let coverage_result = client.coverage();
    assert_eq!(coverage_result, vec![&env, 1, 1]);
}

#[test]
fn test_num_fns_5() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let num_fns: u32 = 5;
    let features = vec![&env, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    client.init(&num_fns, &features);

    let calls = [
        (0, 1),
        (0, 2),
        (1, 3),
        (1, 4),
        (2, 5),
        (2, 6),
        (3, 7),
        (3, 8),
        (4, 9),
        (4, 10),
    ];

    for &(fn_i, arg) in calls.iter() {
        client.call(&fn_i, &arg);
    }

    let coverage_result = client.coverage();
    assert_eq!(
        coverage_result,
        vec![&env, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
    );
}