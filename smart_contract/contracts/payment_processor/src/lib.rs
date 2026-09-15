use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol};

#[derive(Clone)]
#[contracttype]
pub struct Payment {
    pub client: Address,
    pub amount: i128,
    pub platform_fee: i128,
    pub is_released: bool,
}

#[contract]
pub struct PaymentProcessorContract;

#[contractimpl]
impl PaymentProcessorContract {
    pub fn initialize(env: &Env, relayer: Address, token: Address) {
        env.storage().instance().set(&Symbol::new(&env, "relayer"), &relayer);
        env.storage().instance().set(&Symbol::new(&env, "token"), &token);
        env.storage().instance().set(&Symbol::new(&env, "platform_fee_percentage"), &10u32);
        env.storage().instance().set(&Symbol::new(&env, "platform_wallet"), &env.current_contract_address());
        env.storage().instance().set(&Symbol::new(&env, "payment_id"), &0u64);
    }

    fn require_relayer(env: &Env) {
        let relayer: Address = env.storage().instance().get(&Symbol::new(&env, "relayer")).unwrap();
        assert_eq!(env.current_contract_address(), relayer, "Caller is not the relayer");
    }

    pub fn create_payment_for(env: &Env, client: Address, amount: i128) {
        assert!(amount > 10_000_000, "Amount must be greater than 10 USDT");

        let token: Address = env.storage().instance().get(&Symbol::new(&env, "token")).unwrap();
        // In a real implementation, you'd check token balance and allowance via cross-contract call
        // For now, we'll assume this is checked externally

        let platform_fee_percentage: u32 = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "platform_fee_percentage"))
            .unwrap();
        let platform_fee = (amount * platform_fee_percentage as i128) / 100;

        let current_payment_id: u64 = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "payment_id"))
            .unwrap();
        let new_payment_id = current_payment_id + 1;

        let payment = Payment {
            client: client.clone(),
            amount,
            platform_fee,
            is_released: false,
        };

        env.storage()
            .instance()
            .set(&(Symbol::new(&env, "payment"), new_payment_id), &payment);
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "payment_id"), &new_payment_id);

        env.events()
            .publish((Symbol::new(&env, "payment_created"), new_payment_id, client.clone()), amount);
    }

    pub fn current_payment_id(env: &Env) -> u64 {
        env.storage()
            .instance()
            .get(&Symbol::new(&env, "payment_id"))
            .unwrap()
    }

    pub fn release_artisan_funds_for(env: &Env, artisan: Address, payment_id: u64) {
        Self::require_relayer(env);

        let mut payment: Payment = env
            .storage()
            .instance()
            .get(&(Symbol::new(&env, "payment"), payment_id))
            .unwrap();
        assert!(!payment.is_released, "Payment already released");

        payment.is_released = true;
        let amount_to_artisan = payment.amount - payment.platform_fee;

        let token: Address = env.storage().instance().get(&Symbol::new(&env, "token")).unwrap();
        let platform_wallet: Address = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "platform_wallet"))
            .unwrap();

        // In a real implementation, you'd transfer tokens via cross-contract call
        // For now, we'll assume this is done externally

        let key_spent = (Symbol::new(&env, "amount_spent"), payment.client.clone());
        let current_spent: i128 = env.storage().instance().get(&key_spent).unwrap_or(0);
        env.storage()
            .instance()
            .set(&key_spent, &(current_spent + payment.amount));

        let key_made = (Symbol::new(&env, "amount_made"), artisan.clone());
        let current_made: i128 = env.storage().instance().get(&key_made).unwrap_or(0);
        env.storage()
            .instance()
            .set(&key_made, &(current_made + payment.amount));

        env.storage()
            .instance()
            .set(&(Symbol::new(&env, "payment"), payment_id), &payment);

        env.events()
            .publish((Symbol::new(&env, "payment_released"), payment_id), (amount_to_artisan, payment.platform_fee));
    }

    pub fn refund_client_funds(env: &Env, payment_id: u64) {
        let mut payment: Payment = env
            .storage()
            .instance()
            .get(&(Symbol::new(&env, "payment"), payment_id))
            .unwrap();
        assert!(!payment.is_released, "Payment already released");

        payment.is_released = true;

        let token: Address = env.storage().instance().get(&Symbol::new(&env, "token")).unwrap();
        // In a real implementation, you'd transfer tokens via cross-contract call
        // For now, we'll assume this is done externally

        env.storage()
            .instance()
            .set(&(Symbol::new(&env, "payment"), payment_id), &payment);

        env.events()
            .publish((Symbol::new(&env, "payment_refunded"), payment_id), payment.amount);
    }

    pub fn update_platform_fee(env: &Env, new_fee_percentage: u32) {
        let platform_wallet: Address = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "platform_wallet"))
            .unwrap();
        assert_eq!(
            env.current_contract_address(),
            platform_wallet,
            "Only the platform wallet can update the fee"
        );
        assert!(new_fee_percentage <= 20, "Fee percentage must be between 0 and 20");

        env.storage()
            .instance()
            .set(&Symbol::new(&env, "platform_fee_percentage"), &new_fee_percentage);

        env.events()
            .publish((Symbol::new(&env, "platform_fee_updated"),), new_fee_percentage);
    }

    pub fn get_payment_details(env: &Env, payment_id: u64) -> (Address, i128, i128, bool) {
        let payment: Payment = env
            .storage()
            .instance()
            .get(&(Symbol::new(&env, "payment"), payment_id))
            .unwrap();
        (payment.client, payment.amount, payment.platform_fee, payment.is_released)
    }

    pub fn get_client_amount_spent(env: &Env, client: Address) -> i128 {
        let key = (Symbol::new(&env, "amount_spent"), client);
        env.storage().instance().get(&key).unwrap_or(0)
    }

    pub fn get_artisan_amount_made(env: &Env, artisan: Address) -> i128 {
        let key = (Symbol::new(&env, "amount_made"), artisan);
        env.storage().instance().get(&key).unwrap_or(0)
    }

    pub fn has_client_released_funds(env: &Env, payment_id: u64) -> bool {
        let payment: Payment = env
            .storage()
            .instance()
            .get(&(Symbol::new(&env, "payment"), payment_id))
            .unwrap();
        payment.is_released
    }

    pub fn get_platform_fee_percentage(env: &Env) -> u32 {
        env.storage()
            .instance()
            .get(&Symbol::new(&env, "platform_fee_percentage"))
            .unwrap()
    }
}
