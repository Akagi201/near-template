// The rest of this file holds the inline tests for the code above
// Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
#![cfg(test)]
use super::*;

#[test]
fn get_default_greeting() {
	let contract = Contract::default();
	// this test did not call set_greeting so should return the default "Hello" greeting
	assert_eq!(contract.get_greeting(), "Hello".to_string());
}

#[test]
fn set_then_get_greeting() {
	let mut contract = Contract::default();
	contract.set_greeting("howdy".to_string());
	assert_eq!(contract.get_greeting(), "howdy".to_string());
}
