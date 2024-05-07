use rand::distributions::{Alphanumeric, Distribution};
use rand::{Rng, thread_rng};

fn generate_password(length: usize, use_special_chars: bool, exclude_similar: bool) -> String {
    let mut rng = thread_rng();
    let mut charset = Vec::new();

    charset.extend(Alphanumeric.sample_iter(&mut rng).take(62).map(|c| c as char));

    if use_special_chars {
        let special_chars = "!@#$%^&*()_+-=[]{}|;:,.<>?";
        charset.extend(special_chars.chars());
    }

    if exclude_similar {
        charset.retain(|&c| !matches!(c, '0' | 'O' | '1' | 'I' | 'l'));
    }

    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx]
        })
        .collect();

    password
}

fn main() {
    let password_length = 12;
    let use_special_chars = true;
    let exclude_similar = true;
    let password = generate_password(password_length, use_special_chars, exclude_similar);
    println!("Generated password: {}", password);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_length_test() {
        let password = generate_password(10, false, false);
        assert_eq!(password.len(), 10);
    }

    #[test]
    fn password_special_chars_test() {
        let password = generate_password(10, true, false);
        let has_special = password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c));
        assert!(has_special, "Password should contain special characters");
    }

    #[test]
    fn password_exclude_similar_test() {
        let password = generate_password(50, true, true);
        let has_similar = password.chars().any(|c| matches!(c, '0' | 'O' | '1' | 'I' | 'l'));
        assert!(!has_similar, "Password should not contain similar characters");
    }

    #[test]
    fn password_comprehensive_test() {
        let password = generate_password(100, true, false);
        let has_upper = password.chars().any(|c| c.is_uppercase());
        let has_lower = password.chars().any(|c| c.is_lowercase());
        let has_digit = password.chars().any(|c| c.is_digit(10));
        let has_special = password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c));

        assert!(has_upper, "Password should have at least one uppercase letter");
        assert!(has_lower, "Password should have at least one lowercase letter");
        assert!(has_digit, "Password should have at least one digit");
        assert!(has_special, "Password should have at least one special character");
    }
}
