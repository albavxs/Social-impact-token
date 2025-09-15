#![cfg(test)]

use soroban_sdk::token::TokenClient;
use soroban_sdk::{testutils::Address as _, Address, Env, IntoVal, String};

use impact_contract::{ImpactContract, ImpactContractClient};

#[test]
fn test_donation_flow() {
    let env = Env::default();
    env.mock_all_auths(); // libera autenticação nos testes

    // Criar contas (admin + doador)
    let admin = Address::generate(&env);
    let donor = Address::generate(&env);

    // Carregar o WASM do token padrão do arquivo
    let wasm =
        std::fs::read("wasm/soroban_token_contract.wasm").expect("WASM do token não encontrado");
    let wasm_bytes = soroban_sdk::Bytes::from_slice(&env, &wasm);
    let usdc_id = env.register_contract_wasm(None, wasm_bytes);
    let usdc_client = TokenClient::new(&env, &usdc_id);

    // Inicializar o token USDC
    let admin_sym = soroban_sdk::Symbol::new(&env, "admin");
    let decimal_sym = soroban_sdk::Symbol::new(&env, "decimal");
    let name_sym = soroban_sdk::Symbol::new(&env, "name");
    let symbol_sym = soroban_sdk::Symbol::new(&env, "symbol");
    env.invoke_contract::<()>(
        &usdc_id,
        &soroban_sdk::Symbol::new(&env, "initialize"),
        soroban_sdk::vec![
            &env,
            admin.clone().into_val(&env),
            7i128.into_val(&env),
            String::from_str(&env, "USD Coin").into_val(&env),
            String::from_str(&env, "USDC").into_val(&env)
        ],
    );

    // Mintar USDC pro doador
    env.invoke_contract::<()>(
        &usdc_id,
        &soroban_sdk::Symbol::new(&env, "mint"),
        soroban_sdk::vec![&env, donor.clone().into_val(&env), 1000i128.into_val(&env)],
    );

    // Deploy do contrato
    let contract_id = env.register_contract(None, ImpactContract);
    let client = ImpactContractClient::new(&env, &contract_id);

    // Inicializar contrato
    // Para o teste, vamos usar o mesmo token USDC como TREE
    client.init(&admin, &usdc_id, &usdc_id).unwrap();

    // Doar 500 USDC
    client.donate(&donor, &500).unwrap();

    // Consultar saldo de TREE
    let tree_balance = client.balance(&donor);

    assert_eq!(tree_balance, 500);

    // Conferir se USDC realmente saiu do doador
    let donor_usdc = usdc_client.balance(&donor);
    assert_eq!(donor_usdc, 500);

    // Conferir se USDC entrou no contrato
    let contract_usdc = usdc_client.balance(&contract_id);
    assert_eq!(contract_usdc, 500);
}
