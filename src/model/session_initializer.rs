use konnektoren_core::session::Session;

pub trait SessionInitializer {
    fn initialize(&self, session: &Session) -> Result<Session, &str>;
}
