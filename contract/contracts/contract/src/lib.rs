#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, symbol_short, Symbol};

#[contract]
pub struct SchoolLoyaltyContract;

#[contractimpl]
impl SchoolLoyaltyContract {
    // 1. Phải gọi hàm này TRƯỚC khi làm bất cứ việc gì khác
    pub fn init(env: Env, admin: Address) {
        if env.storage().instance().has(&symbol_short!("admin")) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&symbol_short!("admin"), &admin);
    }

    pub fn reward_student(env: Env, student: Address, amount: i128) {
        // Kiểm tra admin an toàn hơn
        let admin: Address = env.storage().instance()
            .get(&symbol_short!("admin"))
            .expect("Contract not initialized! Please call init first.");
        
        admin.require_auth();

        // Lưu trữ số dư
        let mut balance: i128 = env.storage().persistent().get(&student).unwrap_or(0);
        balance += amount;
        env.storage().persistent().set(&student, &balance);
    }
}