#![no_std]

use soroban_sdk::token::TokenClient;
use soroban_sdk::{contract, contractimpl, Address, Env, IntoVal};

mod error;
mod events;
mod storage;

use error::ContractError;
use events::*;
use storage::*;

#[contract]
pub struct ImpactContract;

#[contractimpl]
impl ImpactContract {
    pub fn init(
        env: Env,
        admin: Address,
        usdc: Address,
        tree: Address,
    ) -> Result<(), ContractError> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(ContractError::AlreadyInitialized);
        }

        set_admin(&env, &admin);
        set_usdc(&env, &usdc);

        // Recebe endereço do token TREE já criado
        set_tree(&env, &tree);

        init_event(&env, &admin);
        Ok(())
    }

    pub fn donate(env: Env, donor: Address, amount: i128) -> Result<(), ContractError> {
        if amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }

        let usdc = get_usdc(&env);
        let tree = get_tree(&env);

        // Transfere USDC
        let usdc_client = TokenClient::new(&env, &usdc);
        usdc_client.transfer(&donor, &env.current_contract_address(), &amount);

        // Emite TREE (mint) - contrato precisa ser admin do token TREE
        // Mint TREE usando invoke_contract
        let symbol = soroban_sdk::Symbol::new(&env, "mint");
        let args = soroban_sdk::vec![&env, donor.into_val(&env), amount.into_val(&env)];
        env.invoke_contract::<()>(&tree, &symbol, args);

        donation_event(&env, &donor, amount);
        Ok(())
    }

    pub fn balance(env: Env, user: Address) -> i128 {
        let tree = get_tree(&env);
        let tree_client = TokenClient::new(&env, &tree);
        tree_client.balance(&user)
    }
}
