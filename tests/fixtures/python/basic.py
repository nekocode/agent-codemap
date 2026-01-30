"""
Python 测试用例: 基础类和函数
"""


class User:
    """用户类"""

    def __init__(self, name: str, email: str):
        self.name = name
        self.email = email

    def validate_email(self) -> bool:
        return "@" in self.email

    def to_dict(self) -> dict:
        return {"name": self.name, "email": self.email}


class Session:
    """会话类"""

    @classmethod
    def create(cls, user: User) -> "Session":
        return cls()

    @staticmethod
    def generate_token() -> str:
        return "token"

    def validate(self, token: str) -> bool:
        return len(token) > 0


def login(username: str, password: str) -> Session:
    """登录函数"""
    return Session()


def logout(session: Session) -> None:
    """登出函数"""
    pass


async def fetch_user(user_id: int) -> User:
    """异步函数"""
    return User("test", "test@example.com")


DEFAULT_TIMEOUT = 30
MAX_RETRIES = 3
