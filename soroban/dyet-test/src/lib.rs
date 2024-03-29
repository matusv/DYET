#![no_std]
use soroban_sdk::{contractimpl, symbol, Env, Symbol, Vec};

pub struct Contract;

const NUM_FNS: Symbol = symbol!("NUM_FNS");
const FEATURES: Symbol = symbol!("FEATURES");
const COVERAGE: Symbol = symbol!("COVERAGE");

fn get_coverage_from_storage(env: &Env) -> Vec<u32> {
    match env.storage().get(&COVERAGE) {
        Some(Ok(c)) => c,
        _ => Vec::new(&env),
    }
}

fn get_features_from_storage(env: &Env) -> Vec<u32> {
    match env.storage().get(&FEATURES) {
        Some(Ok(features)) => features,
        _ => Vec::new(&env),
    }
}

fn update_coverage(env: &Env, cs_copy: &Vec<u32>) {
    env.storage().set(&COVERAGE, cs_copy);
}

#[contractimpl]
impl Contract {
    pub fn init(env: Env, num_fns: u32, features: Vec<u32>) -> (u32, Vec<u32>, Vec<u32>) {
        env.storage().set(&NUM_FNS, &num_fns);
        env.storage().set(&FEATURES, &features);

        let coverage_n: u32 = 2 * num_fns;
        let mut covered_blocks = Vec::<u32>::new(&env);

        for _ in 0..coverage_n {
            covered_blocks.push_back(0);
        }
        update_coverage(&env, &covered_blocks);

        (num_fns, features, covered_blocks)
    }

    pub fn coverage(env: Env) -> Vec<u32> {
        get_coverage_from_storage(&env)
    }

    pub fn features(env: Env) -> Vec<u32> {
        get_features_from_storage(&env)
    }

    pub fn call(env: Env, fn_i: u32, arg: u32) -> Vec<u32> {
        let fs = get_features_from_storage(&env);
        let cs = get_coverage_from_storage(&env);

        let mut cs_copy: Vec<u32> = Vec::new(&env);
        for i in 0..cs.len() {
            cs_copy.push_back(cs.get(i).unwrap_or(Ok(0)).unwrap());
        }
        
        let index0: u32 = 2 * fn_i;
        let index1: u32 = 2 * fn_i + 1;
        if arg == fs.get(index0).unwrap_or(Ok(0)).unwrap() {
            cs_copy.set(index0, 1)
        }

        if arg == fs.get(index1).unwrap_or(Ok(0)).unwrap() {
            cs_copy.set(index1, 1)
        }
        
        update_coverage(&env, &cs_copy);

        cs_copy
    }
}

#[cfg(test)]
mod test;