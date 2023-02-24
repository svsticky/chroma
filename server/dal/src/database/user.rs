use crate::database::{Database, DbResult};

pub struct User<'a> {
    db: &'a Database,
    pub koala_id: u32,
    pub access_token: String,
    pub refresh_token: String,
}

pub struct OAuthAccess {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: i64,
}

impl<'a> User<'a> {
    pub async fn create(db: &'a Database, koala_id: u32, oauth: OAuthAccess, admin: bool) -> DbResult<User<'a>> {
        todo!()
    }

    pub async fn get_by_id(db: &'a Database, koala_id: u32) -> DbResult<Option<User<'a>>> {
        todo!()
    }

    pub async fn create_session(&self) -> DbResult<String> {
        todo!()
    }

    pub async fn get_by_session_id<S: AsRef<str>>(&self, session_id: S) -> DbResult<Option<User<'a>>> {
        todo!()
    }

    pub async fn set_tokens(&mut self, access: String, refresh: String, expires_at: i64) -> DbResult<()> {
        todo!()
    }
}