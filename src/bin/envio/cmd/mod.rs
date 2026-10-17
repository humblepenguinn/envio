pub mod check;
pub mod completion;
pub mod create;
pub mod delete;
pub mod edit;
pub mod export;
pub mod import;
pub mod init;
pub mod keyring;
pub mod list;
pub mod rotate_key;
pub mod run;
pub mod set;
pub mod shell;
pub mod show;
pub mod tui;
pub mod unset;
pub mod version;

use crate::{
    clap_app::{ClapApp, Command},
    diagnostic::DiagnosticReport,
    error::AppResult,
};

impl ClapApp {
    pub fn run(&self) -> AppResult<()> {
        if self.diagnostic {
            DiagnosticReport::generate()?.print()?;
            return Ok(());
        }

        match &self.command {
            Command::Init => init::run(),
            Command::Create {
                profile_name,
                description,
                envs,
                envs_file,
                cipher_kind,
                comments,
                expires,
            } => create::run(
                profile_name,
                description.as_deref(),
                envs.as_deref(),
                envs_file.as_deref(),
                cipher_kind.as_deref(),
                comments.as_deref(),
                expires.as_deref(),
            ),
            Command::Edit { profile_name } => edit::run(profile_name),
            Command::Set {
                profile_name,
                envs,
                comments,
                expires,
            } => set::run(profile_name, envs, comments.as_deref(), expires.as_deref()),
            Command::Unset { profile_name, keys } => unset::run(profile_name, keys),
            Command::Show {
                profile_name,
                no_pretty_print,
                show_comments,
                show_expiration,
            } => show::run(
                profile_name,
                *no_pretty_print,
                *show_comments,
                *show_expiration,
            ),
            Command::List { no_pretty_print } => list::run(*no_pretty_print),
            Command::Delete { profile_name } => delete::run(profile_name),
            Command::Check { profile_name } => check::run(profile_name),
            Command::Export {
                profile_name,
                output_file_path,
                keys,
                format,
            } => export::run(
                profile_name,
                output_file_path.as_deref(),
                keys.as_deref(),
                format,
            ),
            Command::Import {
                source,
                profile_name,
            } => import::run(source, profile_name.as_deref()),
            Command::AddKey { profile_name } => keyring::add_key(profile_name),
            Command::RemoveKey { profile_name } => keyring::remove_key(profile_name),
            Command::RotateKey { profile_name } => rotate_key::run(profile_name),
            Command::Shell { profile_name } => shell::run(profile_name),
            Command::Run {
                profile_name,
                command,
            } => run::run(profile_name, command),
            Command::Tui => tui::run(),
            Command::Completion { shell } => completion::run(shell),
            Command::Version { verbose } => version::run(*verbose),
        }
    }
}
