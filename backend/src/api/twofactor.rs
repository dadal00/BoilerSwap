use super::{
    database::get_user,
    redis::{increment_lock_key, is_redis_locked},
};
use crate::{AppError, AppState};
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    transport::smtp::authentication::Credentials,
};
use once_cell::sync::Lazy;
use rand::{Rng, thread_rng};
use regex::Regex;
use std::sync::Arc;
use tracing::{debug, warn};

pub static CODE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d+$").unwrap());

pub fn generate_code() -> String {
    let mut rng = thread_rng();

    format!("{:06}", rng.gen_range(0..1_000_000))
}

async fn send_code_email(
    state: Arc<AppState>,
    user_email: &str,
    code: &str,
) -> Result<(), AppError> {
    let email = Message::builder()
        .from(format!("BoilerSwap <{}>", state.config.from_email).parse()?)
        .to(user_email.parse()?)
        .subject("BoilerSwap Code")
        .body(format!("Your code is {}", code))?;

    let credentials = Credentials::new(
        state.config.from_email.to_string(),
        state.config.from_email_password.to_string(),
    );

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&state.config.from_email_server)?
        .credentials(credentials)
        .build();

    mailer.send(email).await?;

    Ok(())
}

pub fn spawn_code_task(
    state: Arc<AppState>,
    email: String,
    token: String,
    forgot_key: Option<String>,
) {
    tokio::spawn(async move {
        if forgot_key.is_some() {
            match get_user(state.clone(), &email).await {
                Ok(Some(_)) => (),
                Ok(None) => return,
                Err(_) => return,
            }

            if let Ok(is_ok) = is_redis_locked(
                state.clone(),
                &forgot_key.clone().expect("is_some failed"),
                &email,
                &state.config.verify_max_attempts,
            )
            .await
            {
                if !is_ok {
                    return;
                }
            }
        }

        if let Err(error) = send_code_email(state.clone(), &email, &token).await {
            match error {
                AppError::LettreAddress(msg) => {
                    debug!("Invalid email: {}", msg);
                }
                AppError::LettreTransport(msg) => {
                    debug!("Transport error: {}", msg);
                }
                other => {
                    warn!("Unexpected error: {:?}", other);
                }
            }
        } else if forgot_key.is_some()
            && (increment_lock_key(
                state.clone(),
                &forgot_key.expect("is_some failed"),
                &email,
                &state.config.verify_lock_duration_seconds,
                &state.config.verify_max_attempts,
            )
            .await)
                .is_err()
        {
        }
    });
}
