use super::models::RedisAction;
use axum::http::header::HeaderMap;
use chrono::{DateTime, Datelike, Duration, Utc, Weekday};
use sha2::{Digest, Sha256};
use std::net::IpAddr;

pub fn get_hashed_ip(headers: &HeaderMap, direct_ip: IpAddr) -> String {
    let ip = headers
        .get("cf-connecting-ip")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .or_else(|| {
            headers
                .get("x-forwarded-for")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.split(',').next().map(|s| s.trim().to_string()))
        })
        .unwrap_or_else(|| direct_ip.to_string());

    let mut hasher = Sha256::new();

    hasher.update(ip.as_bytes());

    format!("{:x}", hasher.finalize())
}

pub fn get_key(redis_action: RedisAction, hashed_ip: &str) -> String {
    format!("{}:{}", redis_action.as_ref(), hashed_ip)
}

pub fn get_target_timestamp(target_day: Weekday) -> i64 {
    let now: DateTime<Utc> = Utc::now();

    let today_num = now.weekday().number_from_monday() as i64;
    let target_num = target_day.number_from_monday() as i64;

    let mut days_ahead = (target_num - today_num + 7) % 7;
    if days_ahead == 0 {
        days_ahead = 7;
    }

    let target_date = (now.date_naive() + Duration::days(days_ahead)).and_hms_opt(23, 59, 59);

    target_date
        .expect("time values invalid")
        .and_utc()
        .timestamp()
}

pub fn get_seconds_until(target_day: Weekday) -> i32 {
    let now = Utc::now().timestamp();
    (get_target_timestamp(target_day) - now).try_into().unwrap()
}

pub fn convert_i8_to_u8(payload: &i8) -> u8 {
    payload.checked_abs().unwrap_or(0) as u8
}
