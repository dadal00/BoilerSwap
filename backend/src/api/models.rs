use chrono::NaiveDate;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumString};
use uuid::Uuid;

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

#[derive(EnumString, AsRefStr, PartialEq)]
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

    #[strum(serialize = "code_lock")]
    LockedCode,

    #[strum(serialize = "item_lock")]
    LockedItems,

    #[strum(serialize = "item_deleted")]
    DeletedItem,
}

#[derive(Deserialize)]
pub struct DummyClaims {}

#[derive(TryFromPrimitive, Serialize, Deserialize, EnumString, AsRefStr)]
#[repr(u8)]
pub enum ItemType {
    #[strum(serialize = "Furniture")]
    Furniture = 0,

    #[strum(serialize = "Electronics")]
    Electronics = 1,

    #[strum(serialize = "Books")]
    Books = 2,

    #[strum(serialize = "Kitchen")]
    Kitchen = 3,

    #[strum(serialize = "Clothing")]
    Clothing = 4,

    #[strum(serialize = "Other")]
    Other = 5,

    #[strum(serialize = "Decor")]
    Decor = 6,
}

#[derive(TryFromPrimitive, Serialize, Deserialize, EnumString, AsRefStr)]
#[repr(u8)]
pub enum Condition {
    #[strum(serialize = "Excellent")]
    Excellent = 0,

    #[strum(serialize = "Good")]
    Good = 1,

    #[strum(serialize = "Fair")]
    Fair = 2,
}

#[derive(TryFromPrimitive, Serialize, Deserialize, EnumString, AsRefStr)]
#[repr(u8)]
pub enum Location {
    #[strum(serialize = "CaryQuadEast")]
    CaryQuadEast = 0,

    #[strum(serialize = "WileyHall")]
    WileyHall = 1,

    #[strum(serialize = "HarrisonHall")]
    HarrisonHall = 2,

    #[strum(serialize = "EarhartHall")]
    EarhartHall = 3,

    #[strum(serialize = "HillenbrandHall")]
    HillenbrandHall = 4,

    #[strum(serialize = "ThirdStreetSuites")]
    ThirdStreetSuites = 5,
}

#[derive(TryFromPrimitive, Serialize, Deserialize, EnumString, AsRefStr)]
#[repr(u8)]
pub enum Emoji {
    #[strum(serialize = "Chair")]
    Chair = 0,

    #[strum(serialize = "Snowflake")]
    Snowflake = 1,

    #[strum(serialize = "Books")]
    Books = 2,

    #[strum(serialize = "Pan")]
    Pan = 3,

    #[strum(serialize = "Monitor")]
    Monitor = 4,

    #[strum(serialize = "Decor")]
    Decor = 5,
}

#[derive(Serialize, Deserialize)]
pub struct ItemPayload {
    pub item_type: ItemType,
    pub condition: Condition,
    pub title: String,
    pub description: String,
    pub location: Location,
    pub emoji: Emoji,
}

#[derive(Serialize)]
pub struct Item {
    pub item_id: Uuid,
    pub item_type: String,
    pub title: String,
    pub condition: String,
    pub location: String,
    pub description: String,
    pub emoji: String,
    pub expiration_date: String,
}

pub struct CronItem {
    pub item_id: Uuid,
    pub expiration_date: NaiveDate,
}

pub type ItemRow<'a> = (Uuid, i8, &'a str, i8, i8, &'a str, i8, NaiveDate);

pub type CronItemRow<'a> = (Uuid, NaiveDate);
