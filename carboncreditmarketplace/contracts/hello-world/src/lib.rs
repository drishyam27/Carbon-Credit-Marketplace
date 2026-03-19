#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, panic_with_error};

// Define the data keys for our contract storage
#[contracttype]
pub enum DataKey {
    Admin,
    Balance(Address),
    Listing(u32),
    ListingCount,
}

// Structure to hold marketplace listings
#[contracttype]
pub struct Listing {
    pub seller: Address,
    pub amount: u32,
    pub price: u32, // Price per credit (e.g., in stroops)
    pub active: bool,
}

#[contract]
pub struct CarbonMarketplace;

#[contractimpl]
impl CarbonMarketplace {
    /// Initializes the marketplace with an admin who can mint credits.
    pub fn init(env: Env, admin: Address) {
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::ListingCount, &0u32);
    }

    /// Admin mints certified carbon credits to a specific project/address.
    pub fn mint(env: Env, to: Address, amount: u32) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let mut balance: u32 = env.storage().persistent().get(&DataKey::Balance(to.clone())).unwrap_or(0);
        balance += amount;
        env.storage().persistent().set(&DataKey::Balance(to), &balance);
    }

    /// Retrieves the carbon credit balance of a given address.
    pub fn get_balance(env: Env, user: Address) -> u32 {
        env.storage().persistent().get(&DataKey::Balance(user)).unwrap_or(0)
    }

    /// Allows a user to list their carbon credits for sale.
    pub fn list_credits(env: Env, seller: Address, amount: u32, price: u32) -> u32 {
        seller.require_auth();

        let mut balance: u32 = env.storage().persistent().get(&DataKey::Balance(seller.clone())).unwrap_or(0);
        if balance < amount {
            panic!("Insufficient carbon credits to list.");
        }

        // Deduct from the seller's available balance (acting as an escrow)
        balance -= amount;
        env.storage().persistent().set(&DataKey::Balance(seller.clone()), &balance);

        let mut count: u32 = env.storage().instance().get(&DataKey::ListingCount).unwrap_or(0);
        count += 1;

        let listing = Listing {
            seller,
            amount,
            price,
            active: true,
        };

        env.storage().persistent().set(&DataKey::Listing(count), &listing);
        env.storage().instance().set(&DataKey::ListingCount, &count);

        count // Returns the listing ID
    }
    
    // Note: A complete `buy_credits` function would require integrating a payment 
    // token (like USDC on Stellar) via cross-contract calls to handle the financial settlement.
}