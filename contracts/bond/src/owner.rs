// owner.rs
use soroban_sdk::{contracterror, contracttype, panic_with_error, Address, Env, Symbol};

#[derive(Clone)]
#[contracttype]
enum OwnerKey {
    Owner,
}

#[derive(Copy, Clone)]
#[contracterror]
#[repr(u32)]
pub enum OwnerError {
    OnlyOwner = 1001,
}

pub fn has_owner(env: &Env) -> bool {
    let key = OwnerKey::Owner;
    env.storage().instance().has(&key)
}

pub fn get_owner(env: &Env) -> Option<Address> {
    let key = OwnerKey::Owner;
    env.storage().instance().get(&key)
}

pub fn set_owner(env: &Env, id: &Address) {
    let key = OwnerKey::Owner;
    env.storage().instance().set(&key, id);
    env.events().publish((Symbol::new(env, "OwnerSet"),), id);
}

pub fn only_owner(env: &Env) {
    let owner = get_owner(env);
    if let Some(owner) = owner {
        owner.require_auth();
    } else {
        panic_with_error!(env, OwnerError::OnlyOwner);
    }
}
