// ============================================================
// Kotlin Test: Basic Symbols
// ============================================================

package com.example.test

// ------------------------------------------------------------
// Interface
// ------------------------------------------------------------
interface Identifiable {
    val id: String
    fun identify(): String
}

interface Cacheable {
    fun cache()
    fun invalidate()
}

// ------------------------------------------------------------
// Class
// ------------------------------------------------------------
class User(
    override val id: String,
    val name: String,
    val email: String
) : Identifiable {

    override fun identify(): String = id

    fun validate(): Boolean {
        return name.isNotEmpty() && email.contains("@")
    }

    companion object {
        fun create(name: String, email: String): User {
            return User(java.util.UUID.randomUUID().toString(), name, email)
        }
    }
}

// ------------------------------------------------------------
// Data Class
// ------------------------------------------------------------
data class Address(
    val street: String,
    val city: String,
    val country: String
)

data class Point(val x: Double, val y: Double) {
    fun distance(other: Point): Double {
        val dx = x - other.x
        val dy = y - other.y
        return kotlin.math.sqrt(dx * dx + dy * dy)
    }
}

// ------------------------------------------------------------
// Enum
// ------------------------------------------------------------
enum class Status {
    ACTIVE,
    INACTIVE,
    PENDING
}

enum class UserRole(val level: Int) {
    ADMIN(3),
    USER(2),
    GUEST(1)
}

// ------------------------------------------------------------
// Object (Singleton)
// ------------------------------------------------------------
object UserRepository {
    private val users = mutableListOf<User>()

    fun add(user: User) {
        users.add(user)
    }

    fun find(id: String): User? {
        return users.find { it.id == id }
    }
}

// ------------------------------------------------------------
// Sealed Class
// ------------------------------------------------------------
sealed class Result<out T> {
    data class Success<T>(val data: T) : Result<T>()
    data class Error(val message: String) : Result<Nothing>()
}

// ------------------------------------------------------------
// Extension Function
// ------------------------------------------------------------
fun User.displayName(): String {
    return "$name <$email>"
}

// ------------------------------------------------------------
// Top-level Functions
// ------------------------------------------------------------
fun createUser(name: String, email: String): User {
    return User.create(name, email)
}

fun greet(user: User) {
    println("Hello, ${user.name}!")
}

private fun internalHelper(): Boolean = true

// ------------------------------------------------------------
// Top-level Properties
// ------------------------------------------------------------
val DEFAULT_NAME = "Guest"
const val MAX_USERS = 100
