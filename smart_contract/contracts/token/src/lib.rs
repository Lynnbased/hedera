use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Map};

const CLAIM_AMOUNT: i128 = 1_000_000_000; // 1000 USDT with 6 decimals

#[contract]
pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    pub fn initialize(env: &Env, admin: Address, relayer: Address) {
        env.storage().instance().set(&Symbol::new(&env, "admin"), &admin);
        env.storage().instance().set(&Symbol::new(&env, "relayer"), &relayer);
        env.storage().instance().set(&Symbol::new(&env, "decimals"), &6u32);
        env.storage().instance().set(&Symbol::new(&env, "name"), &Symbol::new(&env, "USD Tethers"));
        env.storage().instance().set(&Symbol::new(&env, "symbol"), &Symbol::new(&env, "USDT"));
        env.storage().instance().set(&Symbol::new(&env, "balances"), &Map::<Address, i128>::new(&env));
        env.storage().instance().set(&Symbol::new(&env, "allowances"), &Map::<(Address, Address), i128>::new(&env));
    }

    fn require_admin(env: &Env) {
        let admin: Address = env.storage().instance().get(&Symbol::new(&env, "admin")).unwrap();
        assert_eq!(env.invoker().into_address(), admin, "Caller is not the admin");
    }

    fn require_relayer(env: &Env) {
        let relayer: Address = env.storage().instance().get(&Symbol::new(&env, "relayer")).unwrap();
        assert_eq!(env.invoker().into_address(), relayer, "Caller is not the relayer");
    }

    pub fn balance(env: &Env, id: Address) -> i128 {
        let balances: Map<Address, i128> = env.storage().instance().get(&Symbol::new(&env, "balances")).unwrap();
        balances.get(id).unwrap_or(0)
    }

    pub fn allowance(env: &Env, from: Address, spender: Address) -> i128 {
        let allowances: Map<(Address, Address), i128> = env.storage().instance().get(&Symbol::new(&env, "allowances")).unwrap();
        allowances.get((from, spender)).unwrap_or(0)
    }

    pub fn approve(env: &Env, from: Address, spender: Address, amount: i128) {
        assert_eq!(env.current_contract_address(), from, "Caller must be the owner");
        let mut allowances: Map<(Address, Address), i128> = env.storage().instance().get(&Symbol::new(&env, "allowances")).unwrap();
        allowances.set((from, spender), amount);
        env.storage().instance().set(&Symbol::new(&env, "allowances"), &allowances);
    }

    pub fn transfer(env: &Env, from: Address, to: Address, amount: i128) {
        assert_eq!(env.current_contract_address(), from, "Caller must be the owner");
        Self::internal_transfer(env, from, to, amount);
    }

    pub fn transfer_from(env: &Env, spender: Address, from: Address, to: Address, amount: i128) {
        let mut allowances: Map<(Address, Address), i128> = env.storage().instance().get(&Symbol::new(&env, "allowances")).unwrap();
        let current_allowance = allowances.get((from, spender.clone())).unwrap_or(0);
        assert!(current_allowance >= amount, "Insufficient allowance");
        allowances.set((from, spender), current_allowance - amount);
        env.storage().instance().set(&Symbol::new(&env, "allowances"), &allowances);
        Self::internal_transfer(env, from, to, amount);
    }

    fn internal_transfer(env: &Env, from: Address, to: Address, amount: i128) {
        let mut balances: Map<Address, i128> = env.storage().instance().get(&Symbol::new(&env, "balances")).unwrap();
        let from_balance = balances.get(from.clone()).unwrap_or(0);
        assert!(from_balance >= amount, "Insufficient balance");
        balances.set(from.clone(), from_balance - amount);
        let to_balance = balances.get(to.clone()).unwrap_or(0);
        balances.set(to, to_balance + amount);
        env.storage().instance().set(&Symbol::new(&env, "balances"), &balances);
    }

    pub fn mint(env: &Env, to: Address, amount: i128) {
        Self::require_admin(env);
        let mut balances: Map<Address, i128> = env.storage().instance().get(&Symbol::new(&env, "balances")).unwrap();
        let balance = balances.get(to.clone()).unwrap_or(0);
        balances.set(to, balance + amount);
        env.storage().instance().set(&Symbol::new(&env, "balances"), &balances);
    }

    pub fn burn(env: &Env, from: Address, amount: i128) {
        assert_eq!(env.current_contract_address(), from, "Caller must be the owner");
        let mut balances: Map<Address, i128> = env.storage().instance().get(&Symbol::new(&env, "balances")).unwrap();
        let balance = balances.get(from.clone()).unwrap_or(0);
        assert!(balance >= amount, "Insufficient balance");
        balances.set(from, balance - amount);
        env.storage().instance().set(&Symbol::new(&env, "balances"), &balances);
    }

    pub fn approve_for(env: &Env, owner: Address, spender: Address, amount: i128) {
        Self::require_relayer(env);
        let mut allowances: Map<(Address, Address), i128> = env.storage().instance().get(&Symbol::new(&env, "allowances")).unwrap();
        allowances.set((owner, spender), amount);
        env.storage().instance().set(&Symbol::new(&env, "allowances"), &allowances);
    }

    pub fn claim_for(env: &Env, user: Address) {
        Self::require_relayer(env);

        let key = (Symbol::new(&env, "has_claimed"), user.clone());
        let has_claimed: bool = env.storage().instance().get(&key).unwrap_or(false);
        assert!(!has_claimed, "Already claimed");

        env.storage().instance().set(&key, &true);
        Self::mint(env, user, CLAIM_AMOUNT);
    }

    pub fn has_claimed(env: &Env, user: Address) -> bool {
        let key = (Symbol::new(&env, "has_claimed"), user);
        env.storage().instance().get(&key).unwrap_or(false)
    }

    pub fn decimals(env: &Env) -> u32 {
        env.storage().instance().get(&Symbol::new(&env, "decimals")).unwrap()
    }

    pub fn name(env: &Env) -> Symbol {
        env.storage().instance().get(&Symbol::new(&env, "name")).unwrap()
    }

    pub fn symbol(env: &Env) -> Symbol {
        env.storage().instance().get(&Symbol::new(&env, "symbol")).unwrap()
    }
}
