use super::{
    database::insert_user,
    models::{Action, RedisAccount, RedisAction},
    redis::{increment_lock_key, insert_id, insert_session},
    twofactor::spawn_code_task,
};
use crate::{AppError, AppState};
use axum::http::{
    HeaderValue,
    header::{HeaderMap, SET_COOKIE},
};
use axum_extra::extract::CookieJar;
use cookie::{Cookie, CookieJar as cookieCookieJar, SameSite::Strict, time::Duration};
use once_cell::sync::Lazy;
use std::sync::Arc;
use uuid::Uuid;

static COOKIES_TO_CLEAR: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        RedisAction::Session.as_ref(),
        RedisAction::Forgot.as_ref(),
        RedisAction::Update.as_ref(),
        RedisAction::Auth.as_ref(),
    ]
});

pub static CLEARED_COOKIES: Lazy<cookieCookieJar> = Lazy::new(|| {
    let mut jar = cookieCookieJar::new();

    for &old_cookie in COOKIES_TO_CLEAR.iter() {
        let expired = Cookie::build(old_cookie)
            .path("/")
            .http_only(true)
            .secure(true)
            .same_site(Strict)
            .max_age(Duration::seconds(0));
        jar.add(expired);
    }

    jar
});

pub fn generate_cookie(key: &str, value: &str, ttl: i64) -> HeaderMap {
    let mut jar = CLEARED_COOKIES.clone();

    let new_cookie = Cookie::build((key.to_owned(), value.to_owned()))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(Strict)
        .max_age(Duration::seconds(ttl));

    jar.add(new_cookie);

    let mut headers = HeaderMap::new();

    for cookie in jar.delta() {
        headers.append(
            SET_COOKIE,
            HeaderValue::from_str(&cookie.to_string()).unwrap(),
        );
    }

    headers
}

pub fn get_cookie(headers: &HeaderMap, key: &str) -> Option<String> {
    CookieJar::from_headers(headers)
        .get(key)
        .map(|cookie| cookie.value().to_string())
}

pub async fn create_temporary_session(
    state: Arc<AppState>,
    result: &Option<String>,
    redis_account: &RedisAccount,
    redis_action: RedisAction,
    forgot_key: &Option<String>,
    code_key: &Option<String>,
) -> Result<HeaderMap, AppError> {
    if redis_action != RedisAction::Update {
        spawn_code_task(
            state.clone(),
            redis_account.email.clone(),
            redis_account.code.clone(),
            forgot_key.clone(),
        );

        increment_lock_key(
            state.clone(),
            code_key.as_ref().unwrap(),
            &redis_account.email.clone(),
            &state.config.max_codes_duration_seconds,
            &state.config.max_codes,
        )
        .await?;
    }

    let serialized = match result {
        Some(result) => result,
        None => &serde_json::to_string(&redis_account)?,
    };

    let id = Uuid::new_v4().to_string();

    insert_id(
        state.clone(),
        redis_action.as_ref(),
        &id,
        serialized,
        state.config.temporary_session_duration_seconds.into(),
    )
    .await?;

    Ok(generate_cookie(
        redis_action.as_ref(),
        &id,
        state.config.temporary_session_duration_seconds.into(),
    ))
}

pub async fn create_session(
    state: Arc<AppState>,
    redis_account: &RedisAccount,
    redis_action: RedisAction,
    redis_action_secondary: RedisAction,
) -> Result<HeaderMap, AppError> {
    if redis_account.action == Action::Signup {
        insert_user(state.clone(), redis_account.clone()).await?;
    }

    let session_id = Uuid::new_v4().to_string();

    insert_session(
        state.clone(),
        redis_action.as_ref(),
        &session_id,
        redis_action_secondary.as_ref(),
        &redis_account.email,
    )
    .await?;

    Ok(generate_cookie(
        redis_action.as_ref(),
        &session_id,
        state.config.session_duration_seconds.into(),
    ))
}
