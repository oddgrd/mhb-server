use anyhow::Context;
use oauth2::{ClientId, ClientSecret, RedirectUrl};
use shuttle_runtime::SecretStore;

pub struct AppConfig {
    pub client_id: ClientId,
    pub client_secret: ClientSecret,
    pub google_callback_url: RedirectUrl,
}

impl TryFrom<SecretStore> for AppConfig {
    type Error = anyhow::Error;
    fn try_from(value: SecretStore) -> anyhow::Result<Self> {
        let client_id = value
            .get("CLIENT_ID")
            .map(ClientId::new)
            .context("CLIENT_ID secret should be set")?;
        let client_secret = value
            .get("CLIENT_SECRET")
            .map(ClientSecret::new)
            .context("CLIENT_SECRET secret should be set")?;
        let google_callback_url = value
            .get("GOOGLE_CALLBACK_URL")
            .map(RedirectUrl::new)
            .context("GOOGLE_CALLBACK_URL should be valid redirect url")?
            .context("GOOGLE_CALLBACK_URL secret should be set")?;

        Ok(Self {
            client_id,
            client_secret,
            google_callback_url,
        })
    }
}
