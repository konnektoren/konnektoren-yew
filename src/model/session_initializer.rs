use konnektoren_core::session::Session;

pub trait SessionInitializer {
    fn initialize(&self, session: &Session) -> Result<Session, &str>;
}

pub struct DefaultSessionInitializer;

impl SessionInitializer for DefaultSessionInitializer {
    fn initialize(&self, _session: &Session) -> Result<Session, &str> {
        Ok(Session::default())
    }
}
