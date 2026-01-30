// ============================================================
// Swift Test: Basic Symbols
// ============================================================

import Foundation

// ------------------------------------------------------------
// Protocol
// ------------------------------------------------------------
protocol Identifiable {
    var id: String { get }
    func identify() -> String
}

protocol Cacheable {
    func cache()
    func invalidate()
}

// ------------------------------------------------------------
// Class
// ------------------------------------------------------------
class User: Identifiable {
    let id: String
    var name: String
    var email: String

    init(id: String, name: String, email: String) {
        self.id = id
        self.name = name
        self.email = email
    }

    func identify() -> String {
        return id
    }

    func validate() -> Bool {
        return !name.isEmpty && email.contains("@")
    }
}

// ------------------------------------------------------------
// Struct
// ------------------------------------------------------------
struct Point {
    var x: Double
    var y: Double

    func distance(to other: Point) -> Double {
        let dx = x - other.x
        let dy = y - other.y
        return (dx * dx + dy * dy).squareRoot()
    }
}

struct Address {
    let street: String
    let city: String
    let country: String
}

// ------------------------------------------------------------
// Enum
// ------------------------------------------------------------
enum Status {
    case active
    case inactive
    case pending
}

enum Result<T> {
    case success(T)
    case failure(Error)
}

// ------------------------------------------------------------
// Extension
// ------------------------------------------------------------
extension User {
    var displayName: String {
        return "\(name) <\(email)>"
    }
}

// ------------------------------------------------------------
// Typealias
// ------------------------------------------------------------
typealias UserID = String
typealias UserHandler = (User) -> Void

// ------------------------------------------------------------
// Functions
// ------------------------------------------------------------
func createUser(name: String, email: String) -> User {
    return User(id: UUID().uuidString, name: name, email: email)
}

func greet(_ user: User) {
    print("Hello, \(user.name)!")
}

private func internalHelper() -> Bool {
    return true
}
