use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec, Symbol};

#[derive(Clone)]
#[contracttype]
pub struct Artisan {
    pub ipfs_hash: String,
    pub is_verified: bool,
    pub registration_date: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct Client {
    pub ipfs_hash: String,
    pub registration_date: u64,
}

#[contract]
pub struct RegistryContract;

#[contractimpl]
impl RegistryContract {
    pub fn initialize(env: &Env, relayer: Address) {
        env.storage().instance().set(&Symbol::new(&env, "relayer"), &relayer);
        env.storage().instance().set(&Symbol::new(&env, "artisan_addresses"), &Vec::new(&env));
        env.storage().instance().set(&Symbol::new(&env, "client_addresses"), &Vec::new(&env));
    }

    fn require_relayer(env: &Env) {
        let relayer: Address = env.storage().instance().get(&Symbol::new(&env, "relayer")).unwrap();
        assert_eq!(env.invoker().into_address(), relayer, "Caller is not the relayer");
    }

    pub fn register_as_artisan_for(env: &Env, user: Address, ipfs_hash: String) {
        Self::require_relayer(env);

        let key = (Symbol::new(&env, "is_registered_artisan"), user.clone());
        let is_registered: bool = env.storage().instance().get(&key).unwrap_or(false);
        assert!(!is_registered, "User already registered as an artisan");

        let artisan = Artisan {
            ipfs_hash: ipfs_hash.clone(),
            is_verified: false,
            registration_date: env.ledger().timestamp(),
        };

        env.storage().instance().set(&(Symbol::new(&env, "artisan"), user.clone()), &artisan);
        env.storage().instance().set(&key, &true);

        let mut artisan_addresses: Vec<Address> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "artisan_addresses"))
            .unwrap();
        artisan_addresses.push_back(user.clone());
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "artisan_addresses"), &artisan_addresses);

        Self::verify_artisan(env, user.clone());

        env.events()
            .publish((Symbol::new(&env, "artisan_registered"), user.clone()), ipfs_hash);
    }

    pub fn register_as_client_for(env: &Env, user: Address, ipfs_hash: String) {
        Self::require_relayer(env);

        let key = (Symbol::new(&env, "is_registered_client"), user.clone());
        let is_registered: bool = env.storage().instance().get(&key).unwrap_or(false);
        assert!(!is_registered, "User already registered as a client");

        let client = Client {
            ipfs_hash: ipfs_hash.clone(),
            registration_date: env.ledger().timestamp(),
        };

        env.storage().instance().set(&(Symbol::new(&env, "client"), user.clone()), &client);
        env.storage().instance().set(&key, &true);

        let mut client_addresses: Vec<Address> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "client_addresses"))
            .unwrap();
        client_addresses.push_back(user.clone());
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "client_addresses"), &client_addresses);

        env.events()
            .publish((Symbol::new(&env, "client_registered"), user.clone()), ipfs_hash);
    }

    fn verify_artisan(env: &Env, artisan_address: Address) {
        let key = (Symbol::new(&env, "is_registered_artisan"), artisan_address.clone());
        let is_registered: bool = env.storage().instance().get(&key).unwrap_or(false);
        assert!(is_registered, "Not registered as an artisan");

        let mut artisan: Artisan = env
            .storage()
            .instance()
            .get(&(Symbol::new(&env, "artisan"), artisan_address.clone()))
            .unwrap();
        assert!(artisan.registration_date != 0, "Artisan not registered");
        assert!(!artisan.is_verified, "Artisan already verified");

        artisan.is_verified = true;
        env.storage()
            .instance()
            .set(&(Symbol::new(&env, "artisan"), artisan_address.clone()), &artisan);

        env.events()
            .publish((Symbol::new(&env, "artisan_verified"), artisan_address), ());
    }

    pub fn is_artisan_verified(env: &Env, artisan_address: Address) -> bool {
        let key = (Symbol::new(&env, "is_registered_artisan"), artisan_address.clone());
        let is_registered: bool = env.storage().instance().get(&key).unwrap_or(false);
        assert!(is_registered, "Not registered as an artisan");

        let artisan: Artisan = env
            .storage()
            .instance()
            .get(&(Symbol::new(&env, "artisan"), artisan_address))
            .unwrap();
        artisan.is_verified
    }

    pub fn is_client(env: &Env, client_address: Address) -> bool {
        let key = (Symbol::new(&env, "is_registered_client"), client_address);
        env.storage().instance().get(&key).unwrap_or(false)
    }

    pub fn is_artisan(env: &Env, artisan_address: Address) -> bool {
        let key = (Symbol::new(&env, "is_registered_artisan"), artisan_address);
        env.storage().instance().get(&key).unwrap_or(false)
    }

    pub fn get_artisan_details(env: &Env, artisan_address: Address) -> (String, bool, u64) {
        let key = (Symbol::new(&env, "is_registered_artisan"), artisan_address.clone());
        let is_registered: bool = env.storage().instance().get(&key).unwrap_or(false);
        assert!(is_registered, "Not registered as an artisan");

        let artisan: Artisan = env
            .storage()
            .instance()
            .get(&(Symbol::new(&env, "artisan"), artisan_address))
            .unwrap();
        (artisan.ipfs_hash, artisan.is_verified, artisan.registration_date)
    }

    pub fn get_client_details(env: &Env, client_address: Address) -> (String, u64) {
        let key = (Symbol::new(&env, "is_registered_client"), client_address.clone());
        let is_registered: bool = env.storage().instance().get(&key).unwrap_or(false);
        assert!(is_registered, "Not registered as a client");

        let client: Client = env
            .storage()
            .instance()
            .get(&(Symbol::new(&env, "client"), client_address))
            .unwrap();
        (client.ipfs_hash, client.registration_date)
    }

    pub fn get_artisan_count(env: &Env) -> u32 {
        let artisan_addresses: Vec<Address> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "artisan_addresses"))
            .unwrap();
        artisan_addresses.len()
    }

    pub fn get_client_count(env: &Env) -> u32 {
        let client_addresses: Vec<Address> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "client_addresses"))
            .unwrap();
        client_addresses.len()
    }

    pub fn get_all_artisans(env: &Env) -> Vec<Address> {
        env.storage()
            .instance()
            .get(&Symbol::new(&env, "artisan_addresses"))
            .unwrap()
    }

    pub fn get_all_clients(env: &Env) -> Vec<Address> {
        env.storage()
            .instance()
            .get(&Symbol::new(&env, "client_addresses"))
            .unwrap()
    }
}
