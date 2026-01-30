# Ruby 测试用例: 基础结构

# 用户类
class User
  attr_accessor :name, :email
  attr_reader :created_at

  def initialize(name, email)
    @name = name
    @email = email
    @created_at = Time.now
  end

  def validate_email
    email.include?('@')
  end

  def to_hash
    { name: name, email: email }
  end

  def self.create(name, email)
    new(name, email)
  end
end

# 认证模块
module Authenticatable
  def authenticate(token)
    !token.nil? && !token.empty?
  end
end

# 用户服务
class UserService
  include Authenticatable

  def initialize
    @users = []
  end

  def add_user(user)
    @users << user
  end

  def get_user(name)
    @users.find { |u| u.name == name }
  end

  def count
    @users.size
  end

  private

  def validate_user(user)
    user.validate_email
  end
end

# 常量
MAX_USERS = 100
DEFAULT_ROLE = 'user'

# 单例方法
def User.default
  new('default', 'default@example.com')
end

# 别名
class User
  alias_method :valid_email?, :validate_email
end

# 使用示例
if __FILE__ == $0
  service = UserService.new
  user = User.create('Alice', 'alice@example.com')
  service.add_user(user)
  puts service.get_user('Alice').to_hash
end
