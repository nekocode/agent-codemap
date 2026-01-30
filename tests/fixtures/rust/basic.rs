// Rust 测试用例: 基础结构

use std::collections::HashMap;

/// 用户结构体
pub struct User {
    pub name: String,
    pub email: String,
}

/// 认证 trait
pub trait Authenticatable {
    fn authenticate(&self, token: &str) -> bool;
}

/// 用户服务
pub struct UserService {
    users: HashMap<String, User>,
}

impl UserService {
    /// 创建新服务
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    /// 添加用户
    pub fn add_user(&mut self, user: User) {
        self.users.insert(user.name.clone(), user);
    }

    /// 获取用户
    pub fn get_user(&self, name: &str) -> Option<&User> {
        self.users.get(name)
    }
}

impl Authenticatable for UserService {
    fn authenticate(&self, token: &str) -> bool {
        !token.is_empty()
    }
}

/// 常量
pub const MAX_USERS: usize = 100;
pub const DEFAULT_ROLE: &str = "user";

/// 静态变量
static GLOBAL_COUNTER: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);

/// 枚举
pub enum UserRole {
    Admin,
    User,
    Guest,
}

/// 类型别名
pub type UserId = u64;

/// 模块
mod utils {
    pub fn format_name(name: &str) -> String {
        name.to_uppercase()
    }
}

/// 宏定义
macro_rules! create_user {
    ($name:expr, $email:expr) => {
        User {
            name: $name.to_string(),
            email: $email.to_string(),
        }
    };
}

fn main() {
    let mut service = UserService::new();
    let user = create_user!("Alice", "alice@example.com");
    service.add_user(user);
}
