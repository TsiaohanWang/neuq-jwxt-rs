use super::log::prelude::*;
use anyhow::anyhow;
use dotenv::{dotenv, var};

const USERNAME: &str = "NEUQ_USERNAME";
const PASSWORD: &str = "NEUQ_PASSWORD";

pub mod local {
    use super::*;

    pub fn fetch_var() -> (Option<String>, Option<String>) {
        dotenv().ok();

        return if let Ok(username) = var(USERNAME) {
            if let Ok(password) = var(PASSWORD) {
                info!("Environment variables fetched.");
                (Some(username), Some(password))
            } else {
                warn!("Failed to fetch environment variable {}", PASSWORD);
                (Some(username), None)
            }
        } else {
            if let Ok(password) = var(PASSWORD) {
                warn!("Failed to fetch environment variable {}", USERNAME);
                (None, Some(password))
            } else {
                warn!(
                    "Failed to fetch environment variables {} / {}",
                    USERNAME, PASSWORD
                );
                (None, None)
            }
        };
    }

    pub fn env_var() -> anyhow::Result<(String, String)> {
        let res = fetch_var();

        return match res {
            (Some(username), Some(password)) => Ok((username, password)),
            (Some(_username), None) => {
                Err(anyhow!("Environment variable {:?} is not set.", PASSWORD))
            }
            (None, Some(_password)) => {
                Err(anyhow!("Environment variable {:?} is not set.", USERNAME))
            }
            (None, None) => Err(anyhow!(
                "Environment variable {} / {} is not set.",
                USERNAME,
                PASSWORD
            )),
        };
    }
}

pub mod cloudflare {}

pub mod prelude {
    pub use super::local::env_var;
}
