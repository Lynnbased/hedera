use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, Vec};

#[derive(Clone)]
#[contracttype]
pub struct GigInfo {
    pub client: Address,
    pub gig_applicants: Vec<Address>,
    pub hired_artisan: Address,
    pub payment_id: u64,
    pub database_id: [u8; 32],
    pub root_hash: [u8; 32],
    pub artisan_complete: bool,
    pub is_completed: bool,
    pub is_closed: bool,
}

#[contract]
pub struct GigMarketplaceContract;

#[contractimpl]
impl GigMarketplaceContract {
    pub fn initialize(env: &Env, relayer: Address, registry: Address, payment_processor: Address, craft_coin: Address) {
        env.storage().instance().set(&Symbol::new(&env, "relayer"), &relayer);
        env.storage().instance().set(&Symbol::new(&env, "registry"), &registry);
        env.storage().instance().set(&Symbol::new(&env, "payment_processor"), &payment_processor);
        env.storage().instance().set(&Symbol::new(&env, "craft_coin"), &craft_coin);
        env.storage().instance().set(&Symbol::new(&env, "gig_counter"), &0u64);
    }

    fn require_relayer(env: &Env) {
        let relayer: Address = env.storage().instance().get(&Symbol::new(&env, "relayer")).unwrap();
        assert_eq!(env.current_contract_address(), relayer, "Caller is not the relayer");
    }

    pub fn create_gig_for(env: &Env, client: Address, root_hash: [u8; 32], database_id: [u8; 32], budget: i128) {
        Self::require_relayer(env);

        let registry: Address = env.storage().instance().get(&Symbol::new(&env, "registry")).unwrap();
        // In a real implementation, you'd call registry to check if client is registered
        // For now, we'll assume this is checked externally

        let payment_processor: Address = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "payment_processor"))
            .unwrap();
        // In a real implementation, you'd call payment_processor to create payment
        // For now, we'll assume this is done externally

        let current_counter: u64 = env.storage().instance().get(&Symbol::new(&env, "gig_counter")).unwrap();
        let new_gig_id = current_counter + 1;

        let gig = GigInfo {
            client: client.clone(),
            gig_applicants: Vec::new(&env),
            hired_artisan: Address::from_contract_id([0u8; 32]),
            payment_id: new_gig_id, // Simplified - in real implementation, get from payment processor
            database_id,
            root_hash,
            artisan_complete: false,
            is_completed: false,
            is_closed: false,
        };

        env.storage()
            .instance()
            .set(&(Symbol::new(&env, "gig"), new_gig_id), &gig);
        env.storage()
            .instance()
            .set(&(Symbol::new(&env, "index"), database_id), &new_gig_id);
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "gig_counter"), &new_gig_id);

        let key = (Symbol::new(&env, "client_created_gigs"), client.clone());
        let mut gigs: Vec<[u8; 32]> = env.storage().instance().get(&key).unwrap_or(Vec::new(&env));
        gigs.push_back(database_id);
        env.storage().instance().set(&key, &gigs);

        env.events()
            .publish((Symbol::new(&env, "gig_created"), new_gig_id, client.clone()), root_hash);
    }

    pub fn update_gig_info_for(env: &Env, client: Address, database_id: [u8; 32], new_root_hash: [u8; 32]) {
        Self::require_relayer(env);

        let gig_id: u64 = env.storage().instance().get(&(Symbol::new(&env, "index"), database_id)).unwrap();
        let mut gig: GigInfo = env.storage().instance().get(&(Symbol::new(&env, "gig"), gig_id)).unwrap();

        assert_eq!(client, gig.client, "Not gig owner");
        assert!(!gig.is_completed && !gig.is_closed, "Gig finished");

        gig.root_hash = new_root_hash;
        env.storage()
            .instance()
            .set(&(Symbol::new(&env, "gig"), gig_id), &gig);

        env.events()
            .publish((Symbol::new(&env, "gig_state_updated"), gig_id), new_root_hash);
    }

    pub fn get_latest_root_hash(env: &Env) -> [u8; 32] {
        let counter: u64 = env.storage().instance().get(&Symbol::new(&env, "gig_counter")).unwrap();
        let gig: GigInfo = env.storage().instance().get(&(Symbol::new(&env, "gig"), counter)).unwrap();
        gig.root_hash
    }

    pub fn apply_for_gig_for(env: &Env, artisan: Address, database_id: [u8; 32]) {
        Self::require_relayer(env);

        let gig_id: u64 = env.storage().instance().get(&(Symbol::new(&env, "index"), database_id)).unwrap();
        let mut gig: GigInfo = env.storage().instance().get(&(Symbol::new(&env, "gig"), gig_id)).unwrap();

        let counter: u64 = env.storage().instance().get(&Symbol::new(&env, "gig_counter")).unwrap();
        assert!(gig_id <= counter && gig_id != 0, "Invalid gig ID");

        let registry: Address = env.storage().instance().get(&Symbol::new(&env, "registry")).unwrap();
        // In a real implementation, you'd call registry to check if artisan is verified
        // For now, we'll assume this is checked externally

        assert!(!gig.is_closed, "Gig is closed");
        assert!(gig.hired_artisan == Address::from_contract_id([0u8; 32]), "Artisan already hired");
        assert!(!Self::is_applicant(env, gig_id, artisan.clone()), "Already applied");
        assert!(gig.client != artisan, "Cannot apply to your own gig");

        let required_cft = Self::get_required_cft(env, database_id);
        let craft_coin: Address = env.storage().instance().get(&Symbol::new(&env, "craft_coin")).unwrap();
        // In a real implementation, you'd check allowance and burn via cross-contract call
        // For now, we'll assume this is done externally

        gig.gig_applicants.push_back(artisan.clone());
        env.storage()
            .instance()
            .set(&(Symbol::new(&env, "gig"), gig_id), &gig);

        let key = (Symbol::new(&env, "artisan_applied_gigs"), artisan.clone());
        let mut gigs: Vec<[u8; 32]> = env.storage().instance().get(&key).unwrap_or(Vec::new(&env));
        gigs.push_back(database_id);
        env.storage().instance().set(&key, &gigs);

        env.events()
            .publish((Symbol::new(&env, "gig_application_submitted"), gig_id, artisan), ());
    }

    pub fn hire_artisan_for(env: &Env, client: Address, database_id: [u8; 32], artisan: Address) {
        Self::require_relayer(env);

        let gig_id: u64 = env.storage().instance().get(&(Symbol::new(&env, "index"), database_id)).unwrap();
        let mut gig: GigInfo = env.storage().instance().get(&(Symbol::new(&env, "gig"), gig_id)).unwrap();

        let counter: u64 = env.storage().instance().get(&Symbol::new(&env, "gig_counter")).unwrap();
        assert!(gig_id <= counter && gig_id != 0, "Invalid gig ID");
        assert_eq!(client, gig.client, "Not gig owner");
        assert!(gig.hired_artisan == Address::from_contract_id([0u8; 32]), "Artisan already hired");
        assert!(!gig.is_closed, "Gig is closed");
        assert!(Self::is_applicant(env, gig_id, artisan.clone()), "Not an applicant");

        gig.hired_artisan = artisan.clone();
        env.storage()
            .instance()
            .set(&(Symbol::new(&env, "gig"), gig_id), &gig);

        let key_client = (Symbol::new(&env, "client_gig_count"), gig.client.clone());
        let client_count: u32 = env.storage().instance().get(&key_client).unwrap_or(0);
        env.storage().instance().set(&key_client, &(client_count + 1));

        let key_artisan = (Symbol::new(&env, "artisan_hired_count"), artisan.clone());
        let artisan_count: u32 = env.storage().instance().get(&key_artisan).unwrap_or(0);
        env.storage().instance().set(&key_artisan, &(artisan_count + 1));

        env.events()
            .publish((Symbol::new(&env, "artisan_hired"), gig_id, artisan), ());
    }

    pub fn mark_complete_for(env: &Env, artisan: Address, database_id: [u8; 32]) {
        Self::require_relayer(env);

        let gig_id: u64 = env.storage().instance().get(&(Symbol::new(&env, "index"), database_id)).unwrap();
        let mut gig: GigInfo = env.storage().instance().get(&(Symbol::new(&env, "gig"), gig_id)).unwrap();

        let counter: u64 = env.storage().instance().get(&Symbol::new(&env, "gig_counter")).unwrap();
        assert!(gig_id <= counter && gig_id != 0, "Invalid gig ID");
        assert_eq!(artisan, gig.hired_artisan, "Not hired artisan");
        assert!(!gig.is_completed && !gig.is_closed, "Gig finished");
        assert!(!gig.artisan_complete, "Already marked");

        gig.artisan_complete = true;
        env.storage()
            .instance()
            .set(&(Symbol::new(&env, "gig"), gig_id), &gig);

        env.events()
            .publish((Symbol::new(&env, "artisan_mark_completed"), gig_id), ());
    }

    pub fn confirm_complete_for(env: &Env, client: Address, database_id: [u8; 32]) {
        Self::require_relayer(env);

        let gig_id: u64 = env.storage().instance().get(&(Symbol::new(&env, "index"), database_id)).unwrap();
        let mut gig: GigInfo = env.storage().instance().get(&(Symbol::new(&env, "gig"), gig_id)).unwrap();

        let counter: u64 = env.storage().instance().get(&Symbol::new(&env, "gig_counter")).unwrap();
        assert!(gig_id <= counter && gig_id != 0, "Invalid gig ID");
        assert_eq!(client, gig.client, "Not gig owner");
        assert!(gig.artisan_complete && !gig.is_completed && !gig.is_closed, "Gig not completed || Closed");

        gig.is_completed = true;
        env.storage()
            .instance()
            .set(&(Symbol::new(&env, "gig"), gig_id), &gig);

        env.events()
            .publish((Symbol::new(&env, "client_confirm_completed"), gig_id), ());
    }

    pub fn close_gig_for(env: &Env, client: Address, database_id: [u8; 32]) {
        Self::require_relayer(env);

        let gig_id: u64 = env.storage().instance().get(&(Symbol::new(&env, "index"), database_id)).unwrap();
        let mut gig: GigInfo = env.storage().instance().get(&(Symbol::new(&env, "gig"), gig_id)).unwrap();

        let counter: u64 = env.storage().instance().get(&Symbol::new(&env, "gig_counter")).unwrap();
        assert!(gig_id <= counter && gig_id != 0, "Invalid gig ID");
        assert_eq!(client, gig.client, "Not gig owner");
        assert!(gig.hired_artisan == Address::from_contract_id([0u8; 32]), "Cannot close active gig");
        assert!(!gig.is_completed && !gig.is_closed, "Gig already Completed || Closed");

        gig.is_closed = true;
        env.storage()
            .instance()
            .set(&(Symbol::new(&env, "gig"), gig_id), &gig);

        let payment_processor: Address = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "payment_processor"))
            .unwrap();
        // In a real implementation, you'd call payment_processor to refund
        // For now, we'll assume this is done externally

        env.events()
            .publish((Symbol::new(&env, "gig_closed"), gig_id), ());
    }

    pub fn get_required_cft(env: &Env, database_id: [u8; 32]) -> i128 {
        let gig_id: u64 = env.storage().instance().get(&(Symbol::new(&env, "index"), database_id)).unwrap();
        let gig: GigInfo = env.storage().instance().get(&(Symbol::new(&env, "gig"), gig_id)).unwrap();

        let payment_processor: Address = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "payment_processor"))
            .unwrap();
        // In a real implementation, you'd call payment_processor to get budget
        // For now, we'll use a placeholder
        let budget = 100_000_000i128; // Placeholder

        if budget < 100 * 10_000_000 {
            2_0000000 // 2 CFT
        } else if budget < 500 * 10_000_000 {
            5_0000000 // 5 CFT
        } else {
            10_0000000 // 10 CFT
        }
    }

    pub fn get_gig_count(env: &Env) -> u64 {
        env.storage()
            .instance()
            .get(&Symbol::new(&env, "gig_counter"))
            .unwrap()
    }

    pub fn get_gig_info(env: &Env, database_id: [u8; 32]) -> (Address, Address, u64, [u8; 32], bool, bool, bool) {
        let gig_id: u64 = env.storage().instance().get(&(Symbol::new(&env, "index"), database_id)).unwrap();
        let gig: GigInfo = env.storage().instance().get(&(Symbol::new(&env, "gig"), gig_id)).unwrap();

        let counter: u64 = env.storage().instance().get(&Symbol::new(&env, "gig_counter")).unwrap();
        assert!(gig_id <= counter && gig_id != 0, "Invalid gig ID");

        (
            gig.client,
            gig.hired_artisan,
            gig.payment_id,
            gig.root_hash,
            gig.artisan_complete,
            gig.is_completed,
            gig.is_closed,
        )
    }

    pub fn get_gig_applicants(env: &Env, database_id: [u8; 32]) -> Vec<Address> {
        let gig_id: u64 = env.storage().instance().get(&(Symbol::new(&env, "index"), database_id)).unwrap();
        let gig: GigInfo = env.storage().instance().get(&(Symbol::new(&env, "gig"), gig_id)).unwrap();

        let counter: u64 = env.storage().instance().get(&Symbol::new(&env, "gig_counter")).unwrap();
        assert!(gig_id <= counter && gig_id != 0, "Invalid gig ID");

        gig.gig_applicants
    }

    pub fn get_client_gig_count(env: &Env, client: Address) -> u32 {
        let key = (Symbol::new(&env, "client_gig_count"), client);
        env.storage().instance().get(&key).unwrap_or(0)
    }

    pub fn get_artisan_hired_count(env: &Env, artisan: Address) -> u32 {
        let key = (Symbol::new(&env, "artisan_hired_count"), artisan);
        env.storage().instance().get(&key).unwrap_or(0)
    }

    pub fn get_artisan_applied_gigs(env: &Env, artisan: Address) -> Vec<[u8; 32]> {
        let key = (Symbol::new(&env, "artisan_applied_gigs"), artisan);
        env.storage().instance().get(&key).unwrap_or(Vec::new(env))
    }

    pub fn get_client_created_gigs(env: &Env, client: Address) -> Vec<[u8; 32]> {
        let key = (Symbol::new(&env, "client_created_gigs"), client);
        env.storage().instance().get(&key).unwrap_or(Vec::new(env))
    }

    pub fn has_applied_for_gig(env: &Env, artisan: Address, database_id: [u8; 32]) -> bool {
        let gig_id: u64 = env.storage().instance().get(&(Symbol::new(&env, "index"), database_id)).unwrap();
        let counter: u64 = env.storage().instance().get(&Symbol::new(&env, "gig_counter")).unwrap();
        assert!(gig_id <= counter && gig_id != 0, "Invalid gig ID");

        Self::is_applicant(env, gig_id, artisan)
    }

    fn is_applicant(env: &Env, gig_id: u64, artisan: Address) -> bool {
        let gig: GigInfo = env.storage().instance().get(&(Symbol::new(&env, "gig"), gig_id)).unwrap();
        
        let applicants = gig.gig_applicants;
        for i in 0..applicants.len() {
            if applicants.get(i).unwrap() == artisan {
                return true;
            }
        }
        false
    }
}
