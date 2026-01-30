/**
 * Java 测试用例: 基础结构
 */
package com.example;

import java.util.ArrayList;
import java.util.List;

/**
 * 用户类
 */
public class User {
    private String name;
    private String email;
    private int age;

    public User(String name, String email) {
        this.name = name;
        this.email = email;
    }

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public String getEmail() {
        return email;
    }

    public boolean validateEmail() {
        return email != null && email.contains("@");
    }
}

/**
 * 认证接口
 */
interface Authenticatable {
    boolean authenticate(String token);
}

/**
 * 用户服务
 */
class UserService implements Authenticatable {
    private List<User> users = new ArrayList<>();

    public void addUser(User user) {
        users.add(user);
    }

    public User getUser(String name) {
        return users.stream()
            .filter(u -> u.getName().equals(name))
            .findFirst()
            .orElse(null);
    }

    @Override
    public boolean authenticate(String token) {
        return token != null && !token.isEmpty();
    }
}

/**
 * 用户角色枚举
 */
enum UserRole {
    ADMIN,
    USER,
    GUEST
}

/**
 * 注解定义
 */
@interface Validated {
    String value() default "";
}

/**
 * 记录类 (Java 16+)
 */
record UserRecord(String name, String email) {}
