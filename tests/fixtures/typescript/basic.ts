/**
 * TypeScript 测试用例: 基础类型和接口
 */

interface User {
  name: string;
  email: string;
  age?: number;
}

interface Authenticatable {
  authenticate(token: string): boolean;
}

type UserRole = "admin" | "user" | "guest";

type UserWithRole = User & { role: UserRole };

class UserService implements Authenticatable {
  private users: User[] = [];

  constructor() {
    this.users = [];
  }

  addUser(user: User): void {
    this.users.push(user);
  }

  getUser(name: string): User | undefined {
    return this.users.find((u) => u.name === name);
  }

  authenticate(token: string): boolean {
    return token.length > 0;
  }
}

enum Status {
  Active = "active",
  Inactive = "inactive",
  Pending = "pending",
}

const enum Direction {
  Up,
  Down,
  Left,
  Right,
}

function createUser(name: string, email: string): User {
  return { name, email };
}

const validateEmail = (email: string): boolean => {
  return email.includes("@");
};

async function fetchUser(id: number): Promise<User> {
  return { name: "test", email: "test@example.com" };
}

export function login(username: string, password: string): boolean {
  return username.length > 0 && password.length > 0;
}

export class AuthService {
  authenticate(token: string): boolean {
    return token.length > 0;
  }
}

export default UserService;
