// Go 测试用例: 基础结构

package main

import "fmt"

// User 用户结构体
type User struct {
	Name  string
	Email string
	Age   int
}

// Authenticatable 认证接口
type Authenticatable interface {
	Authenticate(token string) bool
}

// UserService 用户服务
type UserService struct {
	users []User
}

// NewUserService 创建用户服务
func NewUserService() *UserService {
	return &UserService{users: make([]User, 0)}
}

// AddUser 添加用户
func (s *UserService) AddUser(user User) {
	s.users = append(s.users, user)
}

// GetUser 获取用户
func (s *UserService) GetUser(name string) *User {
	for _, u := range s.users {
		if u.Name == name {
			return &u
		}
	}
	return nil
}

// Authenticate 实现认证接口
func (s *UserService) Authenticate(token string) bool {
	return len(token) > 0
}

// 常量定义
const (
	MaxUsers    = 100
	DefaultRole = "user"
)

// 变量定义
var (
	globalCounter int
	defaultUser   = User{Name: "default", Email: "default@example.com"}
)

func main() {
	service := NewUserService()
	service.AddUser(User{Name: "Alice", Email: "alice@example.com"})
	fmt.Println(service.GetUser("Alice"))
}
