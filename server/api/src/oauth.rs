use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

use crate::state::Configuration;

pub fn build_oauth_client(
    configuration: Configuration,
    client_id: String,
    client_secret: String,
) -> BasicClient {
    let redirect_url = format!(
        "http://{}:{}/api/auth/google_callback",
        configuration.host, configuration.port
    );

    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
        .expect("Invalid token endpoint URL");

    BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap())
}

#[derive(Clone)]
pub struct OAuthId(pub String);
