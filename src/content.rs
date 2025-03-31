pub mod templates {

    //lib.rs
    pub fn lib_rs() -> &'static str {
        r#"//!
//! Stylus Hello World
//!
//! The following contract implements the Counter example from Foundry.
//!
//! ```solidity
//! contract Counter {
//!     uint256 public number;
//!     function setNumber(uint256 newNumber) public {
//!         number = newNumber;
//!     }
//!     function increment() public {
//!         number++;
//!     }
//! }
//! ```
//!
//! The program is ABI-equivalent with Solidity, which means you can call it from both Solidity and Rust.
//! To do this, run `cargo stylus export-abi`.
//!
//! Note: this code is a template-only and has not been audited.
//!
// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::U256, prelude::*};

// Define some persistent storage using the Solidity ABI.
// `Counter` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct Counter {
        uint256 number;
    }
}

/// Declare that `Counter` is a contract with the following external methods.
#[public]
impl Counter {
    /// Gets the number from storage.
    pub fn number(&self) -> U256 {
        self.number.get()
    }

    /// Sets a number in storage to a user-specified value.
    pub fn set_number(&mut self, new_number: U256) {
        self.number.set(new_number);
    }

    /// Sets a number in storage to a user-specified value.
    pub fn mul_number(&mut self, new_number: U256) {
        self.number.set(new_number * self.number.get());
    }

    /// Sets a number in storage to a user-specified value.
    pub fn add_number(&mut self, new_number: U256) {
        self.number.set(new_number + self.number.get());
    }

    /// Increments `number` and updates its value in storage.
    pub fn increment(&mut self) {
        let number = self.number.get();
        self.set_number(number + U256::from(1));
    }

    /// Adds the wei value from msg_value to the number in storage.
    #[payable]
    pub fn add_from_msg_value(&mut self) {
        let number = self.number.get();
        self.set_number(number + self.vm().msg_value());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_counter() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Counter::from(&vm);

        assert_eq!(U256::ZERO, contract.number());

        contract.increment();
        assert_eq!(U256::from(1), contract.number());

        contract.add_number(U256::from(3));
        assert_eq!(U256::from(4), contract.number());

        contract.mul_number(U256::from(2));
        assert_eq!(U256::from(8), contract.number());

        contract.set_number(U256::from(100));
        assert_eq!(U256::from(100), contract.number());

        // Override the msg value for future contract method invocations.
        vm.set_value(U256::from(2));

        contract.add_from_msg_value();
        assert_eq!(U256::from(102), contract.number());
    }
}
        "#
    }

    // Main.rs template
    pub fn main_rs() -> &'static str {
        r#"#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#[cfg(not(any(test, feature = "export-abi")))]
#[no_mangle]
pub extern "C" fn main() {}

#[cfg(feature = "export-abi")]
fn main() {
    stylus_hello_world::print_abi("MIT-OR-APACHE-2.0", "pragma solidity ^0.8.23;");
}    
        "#
    }

    //env.example
    pub fn env() -> &'static str {
        r#"
RPC_URL=
STYLUS_CONTRACT_ADDRESS=
PRIV_KEY_PATH=
            "#
    }

    // Models module templates
    pub mod models {
        pub fn mod_rs() -> &'static str {
            r#"pub mod user;

// Re-export commonly used items
pub use user::User;"#
        }

        pub fn user_rs() -> &'static str {
            r#"use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
}

impl User {
    pub fn new(id: u32, username: String, email: String) -> Self {
        Self { id, username, email }
    }
    
    pub fn display(&self) -> String {
        format!("User(id={}, username={}, email={})", self.id, self.username, self.email)
    }
}"#
        }
    }

    // Utils module templates
    pub mod utils {
        pub fn mod_rs() -> &'static str {
            r#"pub mod config;
pub mod logger;

// Re-export commonly used items
pub use config::Config;
pub use logger::{info, error, success};"#
        }

        pub fn config_rs() -> &'static str {
            r#"use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub version: String,
    pub description: String,
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&content)?;
        Ok(config)
    }
    
    pub fn get_full_name(&self) -> String {
        format!("{} v{}", self.name, self.version)
    }
}"#
        }

        pub fn logger_rs() -> &'static str {
            r#"pub fn info(message: &str) {
    println!("[INFO] {}", message);
}

pub fn error(message: &str) {
    eprintln!("[ERROR] {}", message);
}

pub fn success(message: &str) {
    println!("[SUCCESS] {}", message);
}"#
        }
    }

    // Controllers module templates
    pub mod controllers {
        pub fn mod_rs() -> &'static str {
            r#"pub mod app;

// Re-export commonly used items
pub use app::App;"#
        }

        pub fn app_rs() -> &'static str {
            r#"use crate::models::User;
use crate::utils::{Config, info};

pub struct App {
    pub config: Config,
    pub users: Vec<User>,
}

impl App {
    pub fn new(config: Config) -> Self {
        info("Initializing application");
        Self {
            config,
            users: Vec::new(),
        }
    }
    
    pub fn add_user(&mut self, user: User) {
        self.users.push(user);
        info(&format!("Added user: {}", user.display()));
    }
    
    pub fn run(&self) {
        println!("Running {} with {} users", self.config.get_full_name(), self.users.len());
        for user in &self.users {
            println!("- {}", user.display());
        }
    }
}"#
        }
    }

    // Views module templates
    pub mod views {
        pub fn mod_rs() -> &'static str {
            r#"pub mod ui;

// Re-export commonly used items
pub use ui::{show_welcome_message, show_menu};"#
        }

        pub fn ui_rs() -> &'static str {
            r#"pub fn show_welcome_message(app_name: &str) {
    println!("=============================================");
    println!("  Welcome to {}!", app_name);
    println!("  Created with Bruno CLI");
    println!("=============================================");
}

pub fn show_menu() -> u32 {
    println!("\nMenu:");
    println!("1. Add a user");
    println!("2. List users");
    println!("3. Exit");
    
    // In a real app, you'd get user input here
    println!("Select an option (simulated: 1)");
    1 // Simulated selection
}"#
        }
    }

    // Configuration files
    pub fn readme_md() -> &'static str {
        r#"# Bruno Project

A project created with Bruno CLI.

## Getting Started

```
cargo build
cargo run
```"#
    }

    pub fn gitignore() -> &'static str {
        r#"/target
.env"#
    }

    pub fn bruno_json() -> &'static str {
        r#"{
    "name": "bruno-project",
    "version": "0.1.0",
    "description": "A project created with Bruno CLI"
}"#
    }
}
