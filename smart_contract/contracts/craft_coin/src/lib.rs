use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Map};

const MINT_INTERVAL: u64 = 30 * 24 * 60 * 60; // 30 days in seconds
const TOKENS_PER_MINT: i128 = 50_0000000; // 50 CFT with 7 decimals

#[contract]
pub struct CraftCoinContract;

#[contractimpl]
impl CraftCoinContract {
    pub fn initialize(env: &Env, admin: Address, relayer: Address, registry: Address) {
        env.storage().instance().set(&Symbol::new(&env, "admin"), &admin);
        env.storage().instance().set(&Symbol::new(&env, "relayer"), &relayer);
        env.storage().instance().set(&Symbol::new(&env, "registry"), &registry);
        env.storage().instance().set(&Symbol::new(&env, "decimals"), &7u32);
        env.storage().instance().set(&Symbol::new(&env, "name"), &Symbol::new(&env, "CraftCoin"));
        env.storage().instance().set(&Symbol::new(&env, "symbol"), &Symbol::new(&env, "CFT"));
        env.storage().instance().set(&Symbol::new(&env, "balances"), &Map::<Address, i128>::new(&env));
        env.storage().instance().set(&Symbol::new(&env, "allowances"), &Map::<(Address, Address), i128>::new(&env));
    }

    fn require_admin(env: &Env) {
        let admin: Address = env.storage().instance().get(&Symbol::new(&env, "admin")).unwrap();
        assert_eq!(env.current_contract_address(), admin, "Caller is not the admin");
    }

    fn require_relayer(env: &Env) {
        let relayer: Address = env.storage().instance().get(&Symbol::new(&env, "relayer")).unwrap();
        assert_eq!(env.current_contract_address(), relayer, "Caller is not the relayer");
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

    pub fn mint_for(env: &Env, user: Address) {
        Self::require_relayer(env);

        let registry: Address = env.storage().instance().get(&Symbol::new(&env, "registry")).unwrap();
        // In a real implementation, you'd call the registry contract to check if user is artisan
        // For now, we'll assume this is checked externally or via cross-contract call
        
        let next_mint_time = Self::next_mint_time(env, user.clone());
        assert!(env.ledger().timestamp() >= next_mint_time, "Cannot mint yet");

        Self::mint(env, user.clone(), TOKENS_PER_MINT);
        
        let key = (Symbol::new(&env, "last_mint"), user.clone());
        env.storage().instance().set(&key, &env.ledger().timestamp());

        env.events()
            .publish((Symbol::new(&env, "minted"), user), TOKENS_PER_MINT);
    }

    pub fn burn_for(env: &Env, user: Address, amount: i128) {
        Self::burn(env, user.clone(), amount);
        env.events()
            .publish((Symbol::new(&env, "burned"), user), amount);
    }

    pub fn can_mint(env: &Env, user: Address) -> bool {
        env.ledger().timestamp() >= Self::next_mint_time(env, user)
    }

    pub fn next_mint_time(env: &Env, user: Address) -> u64 {
        let key = (Symbol::new(&env, "last_mint"), user);
        let last_mint: u64 = env.storage().instance().get(&key).unwrap_or(0);
        if last_mint == 0 {
            return 0; // Can mint immediately if never minted before
        }
        last_mint + MINT_INTERVAL
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
