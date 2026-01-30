// ============================================================
// C# Test: Basic Symbols
// ============================================================

using System;
using System.Collections.Generic;

namespace Example.Test
{
    // --------------------------------------------------------
    // Interface
    // --------------------------------------------------------
    public interface IIdentifiable
    {
        string Id { get; }
        string Identify();
    }

    public interface ICacheable
    {
        void Cache();
        void Invalidate();
    }

    // --------------------------------------------------------
    // Class
    // --------------------------------------------------------
    public class User : IIdentifiable
    {
        public string Id { get; private set; }
        public string Name { get; set; }
        public string Email { get; set; }

        private readonly DateTime _createdAt;

        public User(string name, string email)
        {
            Id = Guid.NewGuid().ToString();
            Name = name;
            Email = email;
            _createdAt = DateTime.UtcNow;
        }

        public string Identify() => Id;

        public bool Validate()
        {
            return !string.IsNullOrEmpty(Name) && Email.Contains("@");
        }

        public static User Create(string name, string email)
        {
            return new User(name, email);
        }
    }

    // --------------------------------------------------------
    // Struct
    // --------------------------------------------------------
    public struct Point
    {
        public double X { get; }
        public double Y { get; }

        public Point(double x, double y)
        {
            X = x;
            Y = y;
        }

        public double Distance(Point other)
        {
            var dx = X - other.X;
            var dy = Y - other.Y;
            return Math.Sqrt(dx * dx + dy * dy);
        }
    }

    public struct Address
    {
        public string Street { get; set; }
        public string City { get; set; }
        public string Country { get; set; }
    }

    // --------------------------------------------------------
    // Enum
    // --------------------------------------------------------
    public enum Status
    {
        Active,
        Inactive,
        Pending
    }

    public enum UserRole
    {
        Admin = 3,
        User = 2,
        Guest = 1
    }

    // --------------------------------------------------------
    // Delegate
    // --------------------------------------------------------
    public delegate void UserHandler(User user);
    public delegate T Factory<T>();

    // --------------------------------------------------------
    // Static Class
    // --------------------------------------------------------
    public static class UserExtensions
    {
        public static string DisplayName(this User user)
        {
            return $"{user.Name} <{user.Email}>";
        }
    }

    // --------------------------------------------------------
    // Abstract Class
    // --------------------------------------------------------
    public abstract class Repository<T>
    {
        protected List<T> Items { get; } = new List<T>();

        public abstract T Find(string id);
        public abstract void Save(T item);

        public virtual void Delete(string id)
        {
            // Default implementation
        }
    }

    // --------------------------------------------------------
    // Record (C# 9+)
    // --------------------------------------------------------
    public record Person(string FirstName, string LastName);
}
