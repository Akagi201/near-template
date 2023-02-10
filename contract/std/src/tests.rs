// The rest of this file holds the inline tests for the code above
// Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
#![cfg(all(test, not(target_arch = "wasm32")))]
#![allow(dead_code, unused)]
use near_sdk::{test_utils::*, testing_env, AccountId, ONE_NEAR};

use super::*;

fn contract_account() -> AccountId {
	"contract".parse::<AccountId>().unwrap()
}

fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
	let mut builder = VMContextBuilder::new();
	builder
		.current_account_id(contract_account())
		.account_balance(15 * ONE_NEAR)
		.signer_account_id(predecessor_account_id.clone())
		.predecessor_account_id(predecessor_account_id);
	builder
}

#[test]
fn test() {}
