use clap::Parser;

#[derive(Parser)]
/*
 * Clap Application
*/
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand, Debug)]
/*
 * The Command enum is a list of all possible subcommands
 * When a subcommand is passed to the application, clap returns the corresponding enum variant
 * The enum variant then calls the run method which is implemented in the command.rs file.
 *
 * When adding a new subcommand, add it to this enum and then write a match arm in the run method
 * which is located in the command.rs file
*/
pub enum Command {
    #[clap(name = "create", about = "Create a new profile")]
    Create {
        #[clap(required = true)]
        profile_name: String,
        #[clap(required = false, long = "file-to-import-envs-from", short = 'f')]
        envs_file: Option<String>,
        #[clap(required = false, long = "envs", short = 'e')]
        envs: Option<Vec<String>>,
        #[clap(required = false, long = "gpg-key-fingerprint", short = 'g')]
        gpg: Option<String>,
    },
    #[clap(name = "add", about = "Add a new environment variable to a profile")]
    Add {
        #[clap(required = true)]
        profile_name: String,
        #[clap(required = true, long = "envs", short = 'e', value_delimiter = ' ')]
        envs: Vec<String>,
    },
    #[clap(name = "load", about = "Load a profile in the current session")]
    Load {
        #[clap(required = true)]
        profile_name: String,
    },
    #[cfg(target_family = "unix")]
    #[clap(name = "unload", about = "Unload a profile from the current session")]
    Unload,
    #[cfg(target_family = "windows")]
    #[clap(name = "unload", about = "Unload a profile from the current session")]
    Unload {
        #[clap(required = true)]
        profile_name: String,
    },
    #[clap(name = "launch", about = "Launch a program with a profile")]
    Launch {
        #[clap(required = true)]
        profile_name: String,
        #[clap(required = true, long = "program", short = 'p', value_delimiter = ' ')]
        program: Vec<String>,
    },
    #[clap(
        name = "remove",
        about = "Remove a environment variable from a profile"
    )]
    Remove {
        #[clap(required = true)]
        profile_name: String,
        #[clap(required = false, long = "envs-to-remove", short = 'e')]
        envs: Option<Vec<String>>,
    },
    #[clap(
        name = "list",
        about = "List all the environment variables in a profile or all the profiles"
    )]
    List {
        #[clap(required = false, long = "profiles", short = 'p')]
        profiles: bool,
        #[clap(required = false, long = "profile-name", short = 'n')]
        profile_name: Option<String>,
        #[clap(required = false, long = "no-pretty-print", short = 'v')]
        no_pretty_print: bool,
    },
    #[clap(name = "update", about = "Update a environment variable in a profile")]
    Update {
        #[clap(required = true)]
        profile_name: String,
        #[clap(required = true, long = "envs", short = 'e')]
        envs: Vec<String>,
    },
    #[clap(
        name = "export",
        about = "Export a profile to a file if no file is specified it will be exported to a file named .env"
    )]
    Export {
        #[clap(required = true)]
        profile_name: String,
        #[clap(required = false, long = "file-to-export-to", short = 'f')]
        file: Option<String>,
    },
    #[clap(name = "import", about = "Import a profile from a file")]
    Import {
        #[clap(required = true)]
        profile_name: String,
        #[clap(required = false, long = "file-to-import-from", short = 'f')]
        file: Option<String>,
        #[clap(required = false, long = "url", short = 'u')]
        url: Option<String>,
    },
    #[clap(name = "version", about = "Print the version")]
    Version {
        #[clap(required = false)]
        verbose: Option<bool>,
    },
}
