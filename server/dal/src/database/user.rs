use sqlx::Type;

mod standard_user;
mod token_user;

pub use standard_user::User as StandardUser;
pub use token_user::User as TokenUser;

#[derive(Clone, Type, Copy)]
#[sqlx(type_name = "user_type")]
pub enum UserType {
    Standard,
    Token,
}

#[derive(Debug)]
pub enum User<'a> {
    Standard(StandardUser<'a>),
    Token(TokenUser<'a>),
}
