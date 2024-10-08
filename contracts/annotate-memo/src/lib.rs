#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, IntoVal, String, Symbol, Val,
    Vec,
};

#[contract]
pub struct Contract;

#[contracttype]
#[derive(Clone)]
pub struct MemoAnnotation {
    auther: Address,
    address: Address,
    memo: String,
}

#[contractimpl]
impl Contract {
    pub fn annotate(
        env: &Env,
        annotations: Vec<MemoAnnotation>,
        invoke_address: Address,
        invoke_func: Symbol,
        invoke_args: Vec<Val>,
    ) -> Val {
        for a in annotations {
            a.auther.require_auth_for_args((a.clone(),).into_val(env));
            env.events().publish((symbol_short!("annotate"),), a);
        }
        env.invoke_contract(&invoke_address, &invoke_func, invoke_args)
    }
}

mod test;
