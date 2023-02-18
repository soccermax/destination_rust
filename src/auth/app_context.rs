pub struct AppContext {
    pub connection_manager: redis::aio::ConnectionManager,
    pub(crate) uaa_public_cert: Option<String>,
}

impl AppContext {
    pub async fn get_uaa_public_cert(&mut self) -> String {
        match self.uaa_public_cert.is_some() {
            true => {
                let copy = self.uaa_public_cert.as_ref().unwrap().to_string();
                copy
            }
            false => {
                let pem = super::auth::get_public_uaa_pem().await;
                let copy = pem.to_string();
                self.uaa_public_cert = Some(pem);
                copy
            }
        }
    }
}

impl Clone for AppContext {
    fn clone(&self) -> Self {
        match self.uaa_public_cert.is_some() {
            true => AppContext {
                uaa_public_cert: Some(self.uaa_public_cert.as_ref().unwrap().to_string()),
                connection_manager: self.connection_manager.clone(),
            },
            false => AppContext {
                uaa_public_cert: None,
                connection_manager: self.connection_manager.clone(),
            },
        }
    }
}
