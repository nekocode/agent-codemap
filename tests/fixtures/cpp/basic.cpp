/**
 * C++ 测试用例: 基础结构
 */

#include <iostream>
#include <string>
#include <vector>
#include <memory>

#define MAX_USERS 100
#define LOG(msg) std::cout << msg << std::endl

namespace app {

// 用户类
class User {
public:
    User(const std::string& name, const std::string& email)
        : name_(name), email_(email) {}

    std::string getName() const { return name_; }
    std::string getEmail() const { return email_; }

    bool validateEmail() const {
        return email_.find('@') != std::string::npos;
    }

private:
    std::string name_;
    std::string email_;
};

// 认证接口
class Authenticatable {
public:
    virtual ~Authenticatable() = default;
    virtual bool authenticate(const std::string& token) = 0;
};

// 用户服务
class UserService : public Authenticatable {
public:
    void addUser(std::unique_ptr<User> user) {
        users_.push_back(std::move(user));
    }

    User* getUser(const std::string& name) {
        for (auto& user : users_) {
            if (user->getName() == name) {
                return user.get();
            }
        }
        return nullptr;
    }

    bool authenticate(const std::string& token) override {
        return !token.empty();
    }

private:
    std::vector<std::unique_ptr<User>> users_;
};

// 模板类
template<typename T>
class Repository {
public:
    void add(T item) {
        items_.push_back(item);
    }

    T* find(size_t index) {
        if (index < items_.size()) {
            return &items_[index];
        }
        return nullptr;
    }

private:
    std::vector<T> items_;
};

// 枚举类
enum class UserRole {
    Admin,
    User,
    Guest
};

// 类型别名
using UserId = uint64_t;

} // namespace app

// 模板函数
template<typename T>
T max(T a, T b) {
    return (a > b) ? a : b;
}

int main() {
    app::UserService service;
    service.addUser(std::make_unique<app::User>("Alice", "alice@example.com"));
    LOG("User service created");
    return 0;
}
