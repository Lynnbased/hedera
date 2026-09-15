use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol};

#[derive(Clone)]
#[contracttype]
pub struct ChatInfo {
    pub root_hash: [u8; 32],
    pub is_active: bool,
}

#[contract]
pub struct ChatSystemContract;

#[contractimpl]
impl ChatSystemContract {
    pub fn initialize(env: &Env, gig_marketplace: Address) {
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "gig_marketplace"), &gig_marketplace);
    }

    pub fn start_conversation_for(
        env: &Env,
        user: Address,
        conversation_id: [u8; 32],
        database_id: [u8; 32],
        initial_root_hash: [u8; 32],
    ) {
        let gig_marketplace: Address = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "gig_marketplace"))
            .unwrap();
        // In a real implementation, you'd call gig_marketplace to get gig info
        // For now, we'll use placeholder values
        let (client, hired_artisan, _, _, _, _, _) = (
            user.clone(),
            Address::from_contract_id([0u8; 32]),
            0u64,
            [0u8; 32],
            false,
            false,
            false,
        );

        assert!(user == client || user == hired_artisan, "Not authorized");

        let key = (Symbol::new(&env, "conversation"), conversation_id);
        let existing_chat: Option<ChatInfo> = env.storage().instance().get(&key);
        assert!(existing_chat.is_none(), "Already exists");

        let chat = ChatInfo {
            root_hash: initial_root_hash,
            is_active: true,
        };

        env.storage().instance().set(&key, &chat);

        env.events()
            .publish((Symbol::new(&env, "conversation_started"), conversation_id), initial_root_hash);
    }

    pub fn update_conversation_for(
        env: &Env,
        user: Address,
        conversation_id: [u8; 32],
        database_id: [u8; 32],
        new_root_hash: [u8; 32],
    ) {
        let key = (Symbol::new(&env, "conversation"), conversation_id);
        let mut chat: ChatInfo = env.storage().instance().get(&key).unwrap();
        assert!(chat.is_active, "Chat not active");

        let gig_marketplace: Address = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "gig_marketplace"))
            .unwrap();
        // In a real implementation, you'd call gig_marketplace to get gig info
        // For now, we'll use placeholder values
        let (client, hired_artisan, _, _, _, _, _) = (
            user.clone(),
            Address::from_contract_id([0u8; 32]),
            0u64,
            [0u8; 32],
            false,
            false,
            false,
        );

        assert!(user == client || user == hired_artisan, "Not authorized");

        chat.root_hash = new_root_hash;
        env.storage().instance().set(&key, &chat);

        env.events()
            .publish((Symbol::new(&env, "conversation_updated"), conversation_id), new_root_hash);
    }

    pub fn get_conversation_details(env: &Env, conversation_id: [u8; 32]) -> ([u8; 32], bool) {
        let key = (Symbol::new(&env, "conversation"), conversation_id);
        let chat: ChatInfo = env.storage().instance().get(&key).unwrap();
        (chat.root_hash, chat.is_active)
    }

    pub fn is_conversation_active(env: &Env, conversation_id: [u8; 32]) -> bool {
        let key = (Symbol::new(&env, "conversation"), conversation_id);
        let chat: Option<ChatInfo> = env.storage().instance().get(&key);
        match chat {
            Some(c) => c.is_active,
            None => false,
        }
    }
}
