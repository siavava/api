//! Tests for study auth primitives: Argon2 password
//! hashing and JWT issue/verify.

use jsonwebtoken::{EncodingKey, Header, encode};
use mongodb::bson::oid::ObjectId;
use server::{
  controllers::study::{
    hash_password, make_token, verify_password, verify_token,
  },
  models::study::{Claims, User},
};

fn make_user() -> User {
  User {
    id: Some(ObjectId::parse_str("507f1f77bcf86cd799439011").unwrap()),
    username: "alice".into(),
    email: "alice@example.com".into(),
    password_hash: String::new(),
    created_time: String::new(),
  }
}

// ---- password hashing -------------------------------------------------------

#[test]
fn hash_and_verify_round_trip() {
  let hash = hash_password("correct horse battery staple").unwrap();
  assert!(verify_password("correct horse battery staple", &hash));
}

#[test]
fn wrong_password_fails_verification() {
  let hash = hash_password("correct horse battery staple").unwrap();
  assert!(!verify_password("wrong horse", &hash));
}

#[test]
fn malformed_hash_fails_verification() {
  assert!(!verify_password("anything", "not-a-phc-string"));
  assert!(!verify_password("anything", ""));
}

#[test]
fn hashes_are_salted() {
  let a = hash_password("same password").unwrap();
  let b = hash_password("same password").unwrap();
  assert_ne!(a, b, "two hashes of one password should differ by salt");
}

// ---- JWT --------------------------------------------------------------------

#[test]
fn token_round_trip_preserves_claims() {
  let user = make_user();
  let token = make_token("secret", &user).unwrap();
  let claims = verify_token("secret", &token).unwrap();
  assert_eq!(claims.sub, "507f1f77bcf86cd799439011");
  assert_eq!(claims.username, "alice");
}

#[test]
fn token_for_unsaved_user_has_empty_sub() {
  let mut user = make_user();
  user.id = None;
  let token = make_token("secret", &user).unwrap();
  let claims = verify_token("secret", &token).unwrap();
  assert_eq!(claims.sub, "");
}

#[test]
fn wrong_secret_is_rejected() {
  let token = make_token("secret", &make_user()).unwrap();
  assert!(verify_token("other-secret", &token).is_err());
}

#[test]
fn garbage_token_is_rejected() {
  assert!(verify_token("secret", "not.a.jwt").is_err());
  assert!(verify_token("secret", "").is_err());
}

#[test]
fn tampered_token_is_rejected() {
  let token = make_token("secret", &make_user()).unwrap();
  let mut tampered = token.clone();
  let payload_start = token.find('.').unwrap() + 1;
  let original = tampered.remove(payload_start);
  let flipped = if original == 'A' { 'B' } else { 'A' };
  tampered.insert(payload_start, flipped);
  assert!(verify_token("secret", &tampered).is_err());
}

#[test]
fn expired_token_is_rejected() {
  let claims = Claims {
    sub: "507f1f77bcf86cd799439011".into(),
    username: "alice".into(),
    exp: 1_000,
  };
  let token = encode(
    &Header::default(),
    &claims,
    &EncodingKey::from_secret(b"secret"),
  )
  .unwrap();
  assert!(verify_token("secret", &token).is_err());
}
