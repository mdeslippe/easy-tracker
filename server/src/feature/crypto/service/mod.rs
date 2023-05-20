use crate::feature::{crypto::model::UserClaims, user::model::User};
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use rand::rngs::OsRng;
use shaku::{Component, Interface};
use time::OffsetDateTime;

/// The algorithm that will be used to encode and decode json web tokens.
const JWT_ALGORITHM: Algorithm = Algorithm::RS256;

/// A crypto service trait.
pub(crate) trait CryptoService: Interface {
    /// # Description
    ///
    /// Create an authentication token for a user.
    ///
    /// # Arguments
    ///
    /// `user` - The user the authentication token is being created for.
    ///
    /// # Returns
    ///
    /// This function returns a result:
    /// - If the token creation was successful, the Ok variant will be returned with the token that
    /// was created.
    /// - If the token creation was not successful, the Err variant will be returned with the error
    /// that occurred.
    fn create_token(&self, user: &User) -> Result<String, jsonwebtoken::errors::Error>;

    /// # Description
    ///
    /// Decode an authentication token.
    ///
    /// # Arguments
    ///
    /// `token` - The authentication token that will be decoded.
    ///
    /// # Returns
    ///
    /// This functions returns a result:
    /// - If the token was successfully decoded, the Ok variant will be returned with the data that
    /// was encoded in the token.
    /// - If the token was not successfully decoded, the Err variant will be returned with the error
    /// that occurred.
    fn decode_token(
        &self,
        token: &String,
    ) -> Result<TokenData<UserClaims>, jsonwebtoken::errors::Error>;

    /// # Description
    ///
    /// Hash a password.
    ///
    /// # Arguments
    ///
    /// `password` - The plain-text password that will be hashed.
    ///
    /// # Returns
    ///
    /// This function returns a result:
    /// - If the password was successfully hashed, the Ok variant will be returned with the password
    /// hash.
    /// - If the password was not successfully hashed, the Err variant will be returned with the
    /// error that occurred.
    fn hash_password(&self, password: &String) -> Result<String, argon2::password_hash::Error>;

    /// # Description
    ///
    /// Check to see if a plain-text password corresponds to a password hash.
    ///
    /// # Arguments
    ///
    /// `password` - The plain-text password that is being verified.
    ///
    /// `hash` - The password hash that the plain-text password is being verified against.
    ///
    /// # Returns
    ///
    /// This function returns a result:
    /// - If the password was successfully verified and is correct, the Ok variant will be returned.
    /// - If the password was not successfully verified or was not correct, the Err variant will be
    /// returned with the error that occurred. Note that an incorrect password will be treated as an
    /// error.
    fn verify_password(
        &self,
        password: &String,
        hash: &String,
    ) -> Result<(), argon2::password_hash::Error>;
}

/// A CryptoServiceImpl struct.
#[derive(Component)]
#[shaku(interface = CryptoService)]
pub(crate) struct CryptoServiceImpl {
    /// The key that will be used to encode tokens.
    jwt_encoding_key: EncodingKey,

    /// The key that will be used to decode tokens.
    jwt_decoding_key: DecodingKey,
}

/// A CryptoService implementation for the CryptoServiceImpl struct.
impl CryptoService for CryptoServiceImpl {
    fn create_token(&self, user: &User) -> Result<String, jsonwebtoken::errors::Error> {
        // Create the token's header.
        let header: Header = Header {
            alg: JWT_ALGORITHM,
            ..Default::default()
        };

        // Create the token's claims.
        let claims: UserClaims = UserClaims {
            id: user.id,
            exp: i64::MAX,
            iat: OffsetDateTime::now_utc().unix_timestamp(),
            password_last_reset: user.password_reset_at.unix_timestamp(),
        };

        // Encode the token and return the result.
        return encode(&header, &claims, &self.jwt_encoding_key);
    }

    fn decode_token(
        &self,
        token: &String,
    ) -> Result<TokenData<UserClaims>, jsonwebtoken::errors::Error> {
        // Create the validation rules.
        let mut validation_rules: Validation = Validation::default();
        validation_rules.validate_exp = false;
        validation_rules.algorithms = vec![JWT_ALGORITHM];

        // Decode the token and return the result.
        return decode(token, &self.jwt_decoding_key, &validation_rules);
    }

    fn hash_password(&self, password: &String) -> Result<String, argon2::password_hash::Error> {
        // Generate a salt.
        let salt: SaltString = SaltString::generate(&mut OsRng);

        // Hash the password.
        let hash: String = Argon2::default()
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        // Return the password hash.
        return Ok(hash);
    }

    fn verify_password(
        &self,
        password: &String,
        hash: &String,
    ) -> Result<(), argon2::password_hash::Error> {
        // Parse the password hash.
        let parsed_hash: PasswordHash = PasswordHash::new(hash)?;

        // Verify the password and return the result.
        return Argon2::default().verify_password(password.as_bytes(), &parsed_hash);
    }
}

/// An implementation for the CryptoServiceImpl struct.
impl CryptoServiceImpl {
    /// # Description
    ///
    /// Create crypto service parameters that can be used to override the default values injected.
    ///
    /// ### Note: This is necessary as the Shaku macro does not make them accessible outside of this
    /// file.
    ///
    /// # Arguments
    ///
    /// `jwt_encoding_key` - The key that will be used to encode jwt tokens.
    ///
    /// `jwt_decoding_key` - The key that will be used to decode jwt tokens.
    ///
    /// # Returns
    ///
    /// The crypto service parameters that were created.
    pub(crate) fn create_parameters(
        jwt_encoding_key: EncodingKey,
        jwt_decoding_key: DecodingKey,
    ) -> CryptoServiceImplParameters {
        return CryptoServiceImplParameters {
            jwt_encoding_key,
            jwt_decoding_key,
        };
    }
}
