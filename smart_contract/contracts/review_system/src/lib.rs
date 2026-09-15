use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, String, Vec};

#[derive(Clone)]
#[contracttype]
pub struct ReviewInfo {
    pub reviewer: Address,
    pub reviewee: Address,
    pub database_id: [u8; 32],
    pub rating: u32,
    pub comment_hash: String,
    pub timestamp: u64,
}

#[contract]
pub struct ReviewSystemContract;

#[contractimpl]
impl ReviewSystemContract {
    pub fn initialize(env: &Env, relayer: Address, registry: Address, gig_marketplace: Address) {
        env.storage().instance().set(&Symbol::new(&env, "relayer"), &relayer);
        env.storage().instance().set(&Symbol::new(&env, "registry"), &registry);
        env.storage().instance().set(&Symbol::new(&env, "gig_marketplace"), &gig_marketplace);
    }

    fn require_relayer(env: &Env) {
        let relayer: Address = env.storage().instance().get(&Symbol::new(&env, "relayer")).unwrap();
        assert_eq!(env.current_contract_address(), relayer, "Caller is not the relayer");
    }

    pub fn client_submit_review_for(
        env: &Env,
        reviewer: Address,
        database_id: [u8; 32],
        rating: u32,
        comment_hash: String,
    ) {
        Self::require_relayer(env);

        assert!(rating >= 1 && rating <= 5, "Rating must be between 1 and 5");

        let gig_marketplace: Address = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "gig_marketplace"))
            .unwrap();
        // In a real implementation, you'd call gig_marketplace to get gig info
        // For now, we'll use placeholder values
        let (client, hired_artisan, _, _, _, is_completed, is_closed) = (
            reviewer.clone(),
            Address::from_contract_id([0u8; 32]),
            0u64,
            [0u8; 32],
            false,
            true,
            false,
        );

        assert_eq!(reviewer, client, "Only the gig client can submit a review");
        assert!(is_completed, "Gig must be completed before submitting a review");
        assert!(!is_closed, "Cannot review a closed gig");

        let new_review = ReviewInfo {
            reviewer: reviewer.clone(),
            reviewee: hired_artisan.clone(),
            database_id,
            rating,
            comment_hash: comment_hash.clone(),
            timestamp: env.ledger().timestamp(),
        };

        let key = (Symbol::new(&env, "artisan_reviews"), hired_artisan.clone());
        let mut reviews: Vec<ReviewInfo> = env.storage().instance().get(&key).unwrap_or(Vec::new(env));
        reviews.push_back(new_review);
        env.storage().instance().set(&key, &reviews);

        env.events()
            .publish((Symbol::new(&env, "review_submitted_by_client"), 
                     reviewer.clone(), hired_artisan.clone(), database_id), rating);
    }

    pub fn artisan_submit_review_for(
        env: &Env,
        reviewer: Address,
        database_id: [u8; 32],
        rating: u32,
        comment_hash: String,
    ) {
        Self::require_relayer(env);

        assert!(rating >= 1 && rating <= 5, "Rating must be between 1 and 5");

        let gig_marketplace: Address = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "gig_marketplace"))
            .unwrap();
        // In a real implementation, you'd call gig_marketplace to get gig info
        // For now, we'll use placeholder values
        let (client, hired_artisan, _, _, _, is_completed, is_closed) = (
            Address::from_contract_id([0u8; 32]),
            reviewer.clone(),
            0u64,
            [0u8; 32],
            false,
            true,
            false,
        );

        assert_eq!(reviewer, hired_artisan, "Only the hired artisan can submit a client review");
        assert!(is_completed, "Gig must be completed before submitting a review");
        assert!(!is_closed, "Cannot review a closed gig");

        let new_review = ReviewInfo {
            reviewer: reviewer.clone(),
            reviewee: client.clone(),
            database_id,
            rating,
            comment_hash: comment_hash.clone(),
            timestamp: env.ledger().timestamp(),
        };

        let key = (Symbol::new(&env, "client_reviews"), client.clone());
        let mut reviews: Vec<ReviewInfo> = env.storage().instance().get(&key).unwrap_or(Vec::new(env));
        reviews.push_back(new_review);
        env.storage().instance().set(&key, &reviews);

        env.events()
            .publish((Symbol::new(&env, "review_submitted_by_artisan"), 
                     reviewer.clone(), client.clone(), database_id), rating);
    }

    pub fn get_artisan_review_infos(env: &Env, artisan: Address) -> Vec<ReviewInfo> {
        let key = (Symbol::new(&env, "artisan_reviews"), artisan);
        env.storage().instance().get(&key).unwrap_or(Vec::new(env))
    }

    pub fn get_client_review_infos(env: &Env, client: Address) -> Vec<ReviewInfo> {
        let key = (Symbol::new(&env, "client_reviews"), client);
        env.storage().instance().get(&key).unwrap_or(Vec::new(env))
    }

    pub fn get_artisan_reviews(env: &Env, artisan: Address) -> Vec<String> {
        let reviews: Vec<ReviewInfo> = Self::get_artisan_review_infos(env, artisan);
        let mut comments = Vec::new(env);
        
        for i in 0..reviews.len() {
            comments.push_back(reviews.get(i).unwrap().comment_hash.clone());
        }
        
        comments
    }

    pub fn get_client_reviews(env: &Env, client: Address) -> Vec<String> {
        let reviews: Vec<ReviewInfo> = Self::get_client_review_infos(env, client);
        let mut comments = Vec::new(env);
        
        for i in 0..reviews.len() {
            comments.push_back(reviews.get(i).unwrap().comment_hash.clone());
        }
        
        comments
    }

    pub fn get_artisan_average_rating(env: &Env, artisan: Address) -> u32 {
        let reviews: Vec<ReviewInfo> = Self::get_artisan_review_infos(env, artisan);
        if reviews.is_empty() {
            return 0;
        }

        let mut total_rating: u32 = 0;
        for i in 0..reviews.len() {
            total_rating += reviews.get(i).unwrap().rating;
        }

        total_rating / reviews.len()
    }

    pub fn get_client_average_rating(env: &Env, client: Address) -> u32 {
        let reviews: Vec<ReviewInfo> = Self::get_client_review_infos(env, client);
        if reviews.is_empty() {
            return 0;
        }

        let mut total_rating: u32 = 0;
        for i in 0..reviews.len() {
            total_rating += reviews.get(i).unwrap().rating;
        }

        total_rating / reviews.len()
    }

    pub fn get_artisan_review_count(env: &Env, artisan: Address) -> u32 {
        let reviews: Vec<ReviewInfo> = Self::get_artisan_review_infos(env, artisan);
        reviews.len()
    }

    pub fn get_client_review_count(env: &Env, client: Address) -> u32 {
        let reviews: Vec<ReviewInfo> = Self::get_client_review_infos(env, client);
        reviews.len()
    }

    pub fn get_review_details(env: &Env, reviewer: Address, reviewee: Address, database_id: [u8; 32]) -> ReviewInfo {
        let key = (Symbol::new(&env, "artisan_reviews"), reviewee.clone());
        let reviews: Vec<ReviewInfo> = env.storage().instance().get(&key).unwrap_or(Vec::new(env));
        
        for i in 0..reviews.len() {
            let review = reviews.get(i).unwrap();
            if review.reviewer == reviewer && review.database_id == database_id {
                return review.clone();
            }
        }
        
        panic!("Review not found");
    }
}
