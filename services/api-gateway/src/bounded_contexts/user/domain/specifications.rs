use regex::Regex;
use lazy_static::lazy_static;

use super::value_objects::{Email, Username};
use super::entities::User;

/// Trait for domain specifications
pub trait Specification<T> {
    fn is_satisfied_by(&self, candidate: &T) -> bool;
}

/// Email validation specification
pub struct EmailSpecification {
    email_regex: Regex,
}

impl EmailSpecification {
    pub fn new() -> Self {
        lazy_static! {
            static ref EMAIL_REGEX: Regex = Regex::new(
                r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
            ).unwrap();
        }
        
        Self {
            email_regex: EMAIL_REGEX.clone(),
        }
    }
}

impl Specification<Email> for EmailSpecification {
    fn is_satisfied_by(&self, email: &Email) -> bool {
        let email_str = email.value();
        
        // Basic format check
        if !self.email_regex.is_match(email_str) {
            return false;
        }
        
        // Length check
        if email_str.len() > 254 {
            return false;
        }
        
        // Local part (before @) cannot be longer than 64 characters
        if let Some(at_pos) = email_str.find('@') {
            let local_part = &email_str[..at_pos];
            if local_part.len() > 64 {
                return false;
            }
        }
        
        // Domain part checks
        if let Some(at_pos) = email_str.find('@') {
            let domain_part = &email_str[at_pos + 1..];
            
            // Domain cannot be longer than 253 characters
            if domain_part.len() > 253 {
                return false;
            }
            
            // Domain must have at least one dot
            if !domain_part.contains('.') {
                return false;
            }
            
            // Each label in domain cannot be longer than 63 characters
            for label in domain_part.split('.') {
                if label.len() > 63 || label.is_empty() {
                    return false;
                }
            }
        }
        
        true
    }
}

/// Username validation specification
pub struct UsernameSpecification {
    username_regex: Regex,
    reserved_usernames: Vec<&'static str>,
}

impl UsernameSpecification {
    pub fn new() -> Self {
        lazy_static! {
            static ref USERNAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
        }
        
        Self {
            username_regex: USERNAME_REGEX.clone(),
            reserved_usernames: vec![
                "admin", "administrator", "root", "system", "user", "guest",
                "api", "www", "mail", "ftp", "support", "help", "info",
                "about", "contact", "terms", "privacy", "legal", "security",
                "vibestream", "vibe", "stream", "music", "artist", "fan",
                "nft", "token", "crypto", "blockchain", "ethereum", "solana",
                "test", "demo", "example", "null", "undefined", "bot",
            ],
        }
    }
}

impl Specification<Username> for UsernameSpecification {
    fn is_satisfied_by(&self, username: &Username) -> bool {
        let username_str = username.value();
        
        // Length check
        if username_str.len() < 3 || username_str.len() > 30 {
            return false;
        }
        
        // Format check
        if !self.username_regex.is_match(username_str) {
            return false;
        }
        
        // Cannot start or end with hyphen or underscore
        if username_str.starts_with('-') || username_str.ends_with('-') ||
           username_str.starts_with('_') || username_str.ends_with('_') {
            return false;
        }
        
        // Cannot have consecutive hyphens or underscores
        if username_str.contains("--") || username_str.contains("__") ||
           username_str.contains("-_") || username_str.contains("_-") {
            return false;
        }
        
        // Check reserved usernames
        let lowercase_username = username_str.to_lowercase();
        if self.reserved_usernames.contains(&lowercase_username.as_str()) {
            return false;
        }
        
        // Cannot be only numbers
        if username_str.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        
        true
    }
}

/// Password strength specification
pub struct PasswordSpecification {
    min_length: usize,
    max_length: usize,
    require_uppercase: bool,
    require_lowercase: bool,
    require_numbers: bool,
    require_special: bool,
    common_passwords: Vec<&'static str>,
}

impl PasswordSpecification {
    pub fn new() -> Self {
        Self {
            min_length: 8,
            max_length: 128,
            require_uppercase: true,
            require_lowercase: true,
            require_numbers: true,
            require_special: true,
            common_passwords: vec![
                "password", "123456", "password123", "admin", "qwerty",
                "letmein", "welcome", "monkey", "1234567890", "abc123",
                "password1", "123456789", "welcome123", "admin123",
                "root", "toor", "pass", "test", "guest", "user",
            ],
        }
    }
    
    pub fn relaxed() -> Self {
        Self {
            min_length: 6,
            max_length: 128,
            require_uppercase: false,
            require_lowercase: true,
            require_numbers: true,
            require_special: false,
            common_passwords: vec![
                "password", "123456", "password123", "admin", "qwerty",
            ],
        }
    }
}

impl Specification<str> for PasswordSpecification {
    fn is_satisfied_by(&self, password: &str) -> bool {
        // Length check
        if password.len() < self.min_length || password.len() > self.max_length {
            return false;
        }
        
        // Character requirements
        let has_uppercase = password.chars().any(|c| c.is_uppercase());
        let has_lowercase = password.chars().any(|c| c.is_lowercase());
        let has_numbers = password.chars().any(|c| c.is_ascii_digit());
        let has_special = password.chars().any(|c| !c.is_alphanumeric());
        
        if self.require_uppercase && !has_uppercase {
            return false;
        }
        
        if self.require_lowercase && !has_lowercase {
            return false;
        }
        
        if self.require_numbers && !has_numbers {
            return false;
        }
        
        if self.require_special && !has_special {
            return false;
        }
        
        // Check against common passwords
        let lowercase_password = password.to_lowercase();
        if self.common_passwords.contains(&lowercase_password.as_str()) {
            return false;
        }
        
        // Check for simple patterns
        if self.is_simple_pattern(password) {
            return false;
        }
        
        true
    }
}

impl PasswordSpecification {
    fn is_simple_pattern(&self, password: &str) -> bool {
        // Check for sequential characters (123456, abcdef)
        let chars: Vec<char> = password.chars().collect();
        if chars.len() >= 4 {
            for i in 0..chars.len() - 3 {
                let slice = &chars[i..i + 4];
                if self.is_sequential_slice(slice) {
                    return true;
                }
            }
        }
        
        // Check for repeated characters (aaaa, 1111)
        if chars.len() >= 4 {
            for i in 0..chars.len() - 3 {
                if chars[i] == chars[i + 1] && chars[i + 1] == chars[i + 2] && chars[i + 2] == chars[i + 3] {
                    return true;
                }
            }
        }
        
        false
    }
    
    fn is_sequential_slice(&self, slice: &[char]) -> bool {
        if slice.len() < 4 {
            return false;
        }
        
        // Check ascending sequence
        let ascending = slice.windows(2).all(|pair| {
            let a = pair[0] as u32;
            let b = pair[1] as u32;
            b == a + 1
        });
        
        // Check descending sequence
        let descending = slice.windows(2).all(|pair| {
            let a = pair[0] as u32;
            let b = pair[1] as u32;
            a == b + 1
        });
        
        ascending || descending
    }
}

/// User active specification
pub struct UserActiveSpecification;

impl UserActiveSpecification {
    pub fn new() -> Self {
        Self
    }
}

impl Specification<User> for UserActiveSpecification {
    fn is_satisfied_by(&self, user: &User) -> bool {
        user.is_active
    }
}

/// User verified specification
pub struct UserVerifiedSpecification;

impl UserVerifiedSpecification {
    pub fn new() -> Self {
        Self
    }
}

impl Specification<User> for UserVerifiedSpecification {
    fn is_satisfied_by(&self, user: &User) -> bool {
        user.is_verified
    }
}

/// Composite specification for combining multiple specifications
pub struct AndSpecification<T> {
    left: Box<dyn Specification<T>>,
    right: Box<dyn Specification<T>>,
}

impl<T> AndSpecification<T> {
    pub fn new(
        left: Box<dyn Specification<T>>,
        right: Box<dyn Specification<T>>,
    ) -> Self {
        Self { left, right }
    }
}

impl<T> Specification<T> for AndSpecification<T> {
    fn is_satisfied_by(&self, candidate: &T) -> bool {
        self.left.is_satisfied_by(candidate) && self.right.is_satisfied_by(candidate)
    }
}

/// Or specification
pub struct OrSpecification<T> {
    left: Box<dyn Specification<T>>,
    right: Box<dyn Specification<T>>,
}

impl<T> OrSpecification<T> {
    pub fn new(
        left: Box<dyn Specification<T>>,
        right: Box<dyn Specification<T>>,
    ) -> Self {
        Self { left, right }
    }
}

impl<T> Specification<T> for OrSpecification<T> {
    fn is_satisfied_by(&self, candidate: &T) -> bool {
        self.left.is_satisfied_by(candidate) || self.right.is_satisfied_by(candidate)
    }
}

/// Not specification
pub struct NotSpecification<T> {
    specification: Box<dyn Specification<T>>,
}

impl<T> NotSpecification<T> {
    pub fn new(specification: Box<dyn Specification<T>>) -> Self {
        Self { specification }
    }
}

impl<T> Specification<T> for NotSpecification<T> {
    fn is_satisfied_by(&self, candidate: &T) -> bool {
        !self.specification.is_satisfied_by(candidate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bounded_contexts::user::domain::value_objects::{Email, Username};

    #[test]
    fn test_email_specification() {
        let spec = EmailSpecification::new();
        
        let valid_email = Email::new("test@example.com".to_string()).unwrap();
        assert!(spec.is_satisfied_by(&valid_email));
        
        let valid_complex_email = Email::new("user+tag@sub.domain.co.uk".to_string()).unwrap();
        assert!(spec.is_satisfied_by(&valid_complex_email));
    }

    #[test]
    fn test_username_specification() {
        let spec = UsernameSpecification::new();
        
        let valid_username = Username::new("validuser123".to_string()).unwrap();
        assert!(spec.is_satisfied_by(&valid_username));
        
        let valid_with_underscore = Username::new("valid_user".to_string()).unwrap();
        assert!(spec.is_satisfied_by(&valid_with_underscore));
        
        // Test reserved username (should fail)
        if let Ok(reserved_username) = Username::new("admin".to_string()) {
            assert!(!spec.is_satisfied_by(&reserved_username));
        }
    }

    #[test]
    fn test_password_specification() {
        let spec = PasswordSpecification::new();
        
        // Valid password
        assert!(spec.is_satisfied_by("SecurePass123!"));
        
        // Invalid passwords
        assert!(!spec.is_satisfied_by("weak")); // Too short
        assert!(!spec.is_satisfied_by("password")); // Common password
        assert!(!spec.is_satisfied_by("PASSWORD123!")); // No lowercase
        assert!(!spec.is_satisfied_by("password123!")); // No uppercase
        assert!(!spec.is_satisfied_by("Password!")); // No numbers
        assert!(!spec.is_satisfied_by("Password123")); // No special chars
        assert!(!spec.is_satisfied_by("Pass1234")); // Sequential pattern
        assert!(!spec.is_satisfied_by("Passaaaa1!")); // Repeated chars
    }

    #[test]
    fn test_relaxed_password_specification() {
        let spec = PasswordSpecification::relaxed();
        
        // Should accept simpler passwords
        assert!(spec.is_satisfied_by("simple123"));
        assert!(spec.is_satisfied_by("test456"));
        
        // Still reject common ones
        assert!(!spec.is_satisfied_by("password"));
        assert!(!spec.is_satisfied_by("123456"));
    }

    #[test]
    fn test_composite_specifications() {
        let active_spec = Box::new(UserActiveSpecification::new());
        let verified_spec = Box::new(UserVerifiedSpecification::new());
        
        let and_spec = AndSpecification::new(active_spec, verified_spec);
        
        // Test with mock user data would go here
        // This test is more conceptual since we'd need actual User instances
    }
} 