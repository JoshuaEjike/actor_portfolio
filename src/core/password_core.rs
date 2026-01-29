use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

pub fn verify_password(plain_password: &str, password_hash: &str) -> Result<(), &'static str> {
    let parsed_hash = PasswordHash::new(password_hash).map_err(|_| "Invalid password hash")?;

    Argon2::default()
        .verify_password(plain_password.as_bytes(), &parsed_hash)
        .map_err(|_| "Invalid credentials")?;

    Ok(())
}

pub fn hash_password(password: &str) -> Result<String, &'static str> {
    Argon2::default()
        .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
        .map_err(|_| "Password hashing failed")
        .map(|hash| hash.to_string())
}
