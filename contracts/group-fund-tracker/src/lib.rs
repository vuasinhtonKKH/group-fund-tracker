#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

// Định nghĩa các khóa (key) dùng để lưu trữ dữ liệu trong Storage
#[contracttype]
pub enum DataKey {
    TotalFund,              // Khóa để lưu tổng số quỹ
    UserBalance(Address),   // Khóa để lưu số tiền đóng góp của từng địa chỉ cụ thể
}

#[contract]
pub struct GroupFundContract;

#[contractimpl]
impl GroupFundContract {
    /// Hàm đóng góp quỹ (Deposit)
    /// Lưu trữ số lượng token mà một người dùng (Address) đóng góp vào sổ quỹ.
    pub fn deposit(env: Env, from: Address, amount: i128) {
        // Yêu cầu xác thực từ người gọi hàm để đảm bảo không ai có thể tự ý 
        // ghi khống số liệu dưới tên của người khác.
        from.require_auth();

        // Kiểm tra hợp lệ: số lượng đóng góp phải lớn hơn 0
        if amount <= 0 {
            panic!("So luong dong gop phai lon hon 0");
        }

        // 1. Cập nhật số tiền của cá nhân người gửi
        let user_key = DataKey::UserBalance(from.clone());
        // Lấy số dư hiện tại trong persistent storage, nếu chưa có thì mặc định là 0
        let mut user_balance: i128 = env.storage().persistent().get(&user_key).unwrap_or(0);
        user_balance += amount;
        // Lưu lại số dư mới cập nhật
        env.storage().persistent().set(&user_key, &user_balance);

        // 2. Cập nhật tổng số tiền của toàn bộ quỹ nhóm
        let mut total_fund: i128 = env.storage().persistent().get(&DataKey::TotalFund).unwrap_or(0);
        total_fund += amount;
        env.storage().persistent().set(&DataKey::TotalFund, &total_fund);
    }

    /// Lấy số dư đóng góp của một người dùng cụ thể
    pub fn get_user_balance(env: Env, user: Address) -> i128 {
        let user_key = DataKey::UserBalance(user);
        env.storage().persistent().get(&user_key).unwrap_or(0)
    }

    /// Lấy tổng số quỹ đang được contract ghi nhận
    pub fn get_total_fund(env: Env) -> i128 {
        env.storage().persistent().get(&DataKey::TotalFund).unwrap_or(0)
    }
}