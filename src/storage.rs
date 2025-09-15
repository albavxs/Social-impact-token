use soroban_sdk::{Address, Env};

#[soroban_sdk::contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Usdc,
    Tree,
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

pub fn get_admin(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::Admin).unwrap()
}

pub fn set_usdc(env: &Env, token: &Address) {
    env.storage().instance().set(&DataKey::Usdc, token);
}

pub fn get_usdc(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::Usdc).unwrap()
}

pub fn set_tree(env: &Env, token: &Address) {
    env.storage().instance().set(&DataKey::Tree, token);
}

pub fn get_tree(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::Tree).unwrap()
}
