// contract.rs
use crate::owner;
use soroban_sdk::{contractimpl, contract, Address, Env};

#[contract]
pub struct MyContract;

#[contractimpl]
impl MyContract {
    // Initialize the contract with an owner
    pub fn init(env: Env, owner: Address) {
        if owner::has_owner(&env) {
            panic!("Owner already set");
        }
        owner::set_owner(&env, &owner);
    }

    // Restricted action, callable only by the owner
    pub fn restricted_action(env: Env) {
        owner::only_owner(&env); // Enforce ownership check
        // Perform restricted logic here
    }

    // Public action accessible to anyone
    pub fn public_action(env: Env) {
        // Example public logic
        env.events().publish(("PublicActionExecuted",), ());
    }

    // Fetch the current owner's address
    pub fn get_owner(env: Env) -> Option<Address> {
        owner::get_owner(&env)
    }
}
