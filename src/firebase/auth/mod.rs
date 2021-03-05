use crate::configuration::FirebaseConfig;
use crate::firebase;

pub static FIREBASE_AUTHENTICATION_AUDIENCE: &str =
    "https://identitytoolkit.googleapis.com/google.identity.identitytoolkit.v1.IdentityToolkit";

pub async fn verify_id_token(
    token: &str,
) -> Result<jsonwebtoken::TokenData<firebase::admin::jwt::Claims>, jsonwebtoken::errors::Error> {
    let firebase_config = FirebaseConfig::new();
    firebase::admin::jwt::verify(token, &firebase_config).await
}
