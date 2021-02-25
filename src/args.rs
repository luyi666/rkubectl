use clap::{Clap, IntoApp};
use clap_generate::{generate, generators::*};

// in line with the [[bin]] name in Cargo.toml
static BIN_NAME: &str = "rkl";
#[derive(Clap, Clone, PartialEq, Debug)]
pub enum Command {
    /// Show description of a pod
    DESCRIBE {name: String},
    /// Delete a pod
    DELETE {name: String},
    /// Show image of a pod
    IMAGE {name: String},
    /// Show docker container id within a pod
    CONTAINER {name: String},
    /// Show log
    LOG {name: String},
    /// Execute a command in a container
    EXEC {name: String},
}
#[derive(Clap, Clone, PartialEq, Debug)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Elvish,
}

#[derive(Clap, Clone, PartialEq, Debug)]
#[clap(version = "0.1", author = "luyi666 <ly921225@gmail.com>")]
pub struct Args {
    /// Generate a SHELL completion script and print to stdout
    #[clap(long, short, arg_enum, value_name = "SHELL")]
    pub completion: Option<Shell>,
    /// Subcommands to show pod info
    #[clap(subcommand)]
    pub cmd: Option<Command>,
    /// Insert a middle name between component and version number (kg2 -> kg-sophon2, with middle name "-sophon")
    #[clap(long, short)]
    pub middle: Option<String>,
}

impl Shell {
    pub fn generate(&self) {
        let mut app = Args::into_app();
        let mut fd = std::io::stdout();
        match self {
            Shell::Bash => generate::<Bash, _>(&mut app, BIN_NAME, &mut fd),
            Shell::Zsh => generate::<Zsh, _>(&mut app, BIN_NAME, &mut fd),
            Shell::Fish => generate::<Fish, _>(&mut app, BIN_NAME, &mut fd),
            Shell::PowerShell => generate::<PowerShell, _>(&mut app, BIN_NAME, &mut fd),
            Shell::Elvish => generate::<Elvish, _>(&mut app, BIN_NAME, &mut fd),
        }
    }
}

#[test]
fn test_command() {
    assert_eq!(
        Args {
            completion: None,
            middle: None,
            cmd: Some(Command::DELETE {name: "sophon".to_string()}),
        },
        Args::parse_from(&["rkl", "delete", "sophon"])
    );
    assert_eq!(
        Args {
            completion: None,
            middle: None,
            cmd: Some(Command::IMAGE {name: "sophon".to_string()}),
        },
        Args::parse_from(&["rkl", "image", "sophon"])
    );
    assert_eq!(
        Args {
            completion: None,
            middle: None,
            cmd: Some(Command::DESCRIBE {name: "sophon".to_string()}),
        },
        Args::parse_from(&["rkl", "describe", "sophon"])
    );
    assert_eq!(
        Args {
            completion: None,
            middle: None,
            cmd: Some(Command::CONTAINER {name: "sophon".to_string()}),
        },
        Args::parse_from(&["rkl", "container", "sophon"])
    );
}