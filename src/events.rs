use soroban_sdk::{Env, Symbol, Address};

pub fn donation_event(env: &Env, donor: &Address, amount: i128) {
    env.events().publish(
        (Symbol::new(env, "Donation"), donor),
        amount,
    );
}

pub fn init_event(env: &Env, admin: &Address) {
    env.events().publish(
        (Symbol::new(env, "Initialized"), admin),
        Symbol::new(env, "OK"),
    );
}
