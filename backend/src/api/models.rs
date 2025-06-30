use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumString};

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub email: String,
    pub password: String,
    pub action: Action,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RedisAccount {
    pub email: String,
    pub action: Action,
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_timestamp: Option<i64>,
    pub password_hash: Option<String>,
}

#[derive(Deserialize)]
pub struct Token {
    pub token: String,
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[derive(Clone)]
pub enum Action {
    Login,
    Signup,
    Forgot,
}

#[derive(Debug, EnumString, AsRefStr, PartialEq)]
pub enum RedisAction {
    #[strum(serialize = "auth_id")]
    Auth,

    #[strum(serialize = "forgot_id")]
    Forgot,

    #[strum(serialize = "locked_timestamp")]
    LockedTime,

    #[strum(serialize = "session_id")]
    Session,

    #[strum(serialize = "temporary_lock")]
    LockedTemporary,

    #[strum(serialize = "update")]
    Update,

    #[strum(serialize = "sessions")]
    SessionStore,

    #[strum(serialize = "verify_lock")]
    LockedVerify,

    #[strum(serialize = "auth_lock")]
    LockedAuth,

    #[strum(serialize = "forgot_lock")]
    LockedForgot,
}
