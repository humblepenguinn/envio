use envio::Env;

use crate::{error::AppResult, profile_ops, prompts, success_msg};

pub fn run(
    profile_name: &str,
    envs: &[String],
    comment: Option<&str>,
    expiration: Option<&str>,
) -> AppResult<()> {
    let mut profile = profile_ops::get_profile_cli(profile_name)?;

    for mut env in parse_envs(profile_name, &profile, envs)? {
        if comment.is_some() {
            env.comment = Some(comment.unwrap_or_default().to_string());
        } else if let Some(prompted) = prompts::text_prompt_if_needed(format!(
            "Enter a comment for '{}' (or leave empty):",
            env.key
        ))? {
            env.comment = Some(prompted);
        }

        if expiration.is_some() {
            env.expiration_date =
                Some(chrono::NaiveDate::parse_from_str(expiration.unwrap(), "%Y-%m-%d")?);
        } else if let Some(prompted) = prompts::date_prompt_if_needed(format!(
            "Select an expiration date for '{}' (or press Enter to skip):",
            env.key
        ))? {
            env.expiration_date = Some(prompted);
        }

        profile.envs.insert(env);
    }

    profile.save()?;
    success_msg!("Changes applied");
    Ok(())
}

fn parse_envs(
    _profile_name: &str,
    profile: &envio::Profile,
    envs: &[String],
) -> AppResult<Vec<Env>> {
    let mut result = Vec::new();

    for raw in envs {
        if raw.contains('=') {
            let mut parts = raw.splitn(2, '=');
            let key = parts.next().unwrap_or("").to_string();
            let value = parts.next().unwrap_or("").to_string();
            result.push(Env::from_key_value(key, value));
        } else {
            let value = prompts::text_prompt(prompts::TextPromptOptions {
                title: format!(
                    "Enter the {}value for {}:",
                    if profile.envs.contains_key(raw) {
                        "new "
                    } else {
                        ""
                    },
                    raw
                ),
                default: None,
            })?;
            result.push(Env::from_key_value(raw.to_string(), value));
        }
    }

    Ok(result)
}
