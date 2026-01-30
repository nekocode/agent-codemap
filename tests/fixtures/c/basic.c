/**
 * C 测试用例: 基础结构
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_USERS 100
#define DEFAULT_NAME "unknown"

#define SQUARE(x) ((x) * (x))

/* 用户结构体 */
typedef struct {
    char name[64];
    char email[128];
    int age;
} User;

/* 用户服务结构体 */
struct UserService {
    User users[MAX_USERS];
    int count;
};

/* 用户角色枚举 */
enum UserRole {
    ROLE_ADMIN,
    ROLE_USER,
    ROLE_GUEST
};

/* 联合体示例 */
union Data {
    int i;
    float f;
    char str[20];
};

/* 函数声明 */
void init_user(User *user, const char *name, const char *email);
User *create_user(const char *name, const char *email);
void free_user(User *user);

/* 函数定义 */
void init_user(User *user, const char *name, const char *email) {
    strncpy(user->name, name, sizeof(user->name) - 1);
    strncpy(user->email, email, sizeof(user->email) - 1);
    user->age = 0;
}

User *create_user(const char *name, const char *email) {
    User *user = (User *)malloc(sizeof(User));
    if (user) {
        init_user(user, name, email);
    }
    return user;
}

void free_user(User *user) {
    free(user);
}

int validate_email(const char *email) {
    return strchr(email, '@') != NULL;
}

int main(int argc, char *argv[]) {
    User *user = create_user("Alice", "alice@example.com");
    printf("User: %s\n", user->name);
    free_user(user);
    return 0;
}
