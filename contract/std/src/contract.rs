// copy from https://github.com/near/create-near-app/blob/master/templates/contracts/rust/src/lib.rs
// Find all our documentation at https://docs.near.org
use near_sdk::{
	borsh::{self, BorshDeserialize, BorshSerialize},
	log, near_bindgen,
};

// Define the default message
const DEFAULT_MESSAGE: &str = "Hello";

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
	message: String,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
	fn default() -> Self {
		Self { message: DEFAULT_MESSAGE.to_string() }
	}
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
	// Public method - returns the greeting saved, defaulting to DEFAULT_MESSAGE
	pub fn get_greeting(&self) -> String {
		self.message.clone()
	}

	// Public method - accepts a greeting, such as "howdy", and records it
	pub fn set_greeting(&mut self, message: String) {
		log!("Saving greeting {}", message);
		self.message = message;
	}
}
