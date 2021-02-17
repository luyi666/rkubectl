use clap::{Clap, IntoApp};
use clap_generate::{generate, generators::*};

static BIN_NAME: &str = "kbl";
#[derive(Clap, Clone, PartialEq, Debug)]
pub enum Command {
    /// show description of a pod
    DESCRIBE {name: String},
    /// delete a pod
    DELETE {name: String},
    /// show image of a pod
    IMAGE {name: String},
    /// show docker container id within a pod
    CONTAINER {name: String},
    /// show owide
    OWIDE {name: String},
    /// show log
    LOG {name: String},
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
pub struct Args {
    /// Generate a SHELL completion script and print to stdout
    #[clap(long, short, arg_enum, value_name = "SHELL")]
    pub completion: Option<Shell>,
    #[clap(subcommand)]
    pub cmd: Option<Command>,
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
fn test_no_cmd() {
    let result = Args::try_parse_from(&["kbl"]);
    assert!(result.is_err());
}

#[test]
fn test_command() {
    assert_eq!(
        Args {
            completion: None,
            cmd: Some(Command::DELETE {name: "sophon".to_string()}),
        },
        Args::parse_from(&["kbl", "delete", "sophon"])
    );
    assert_eq!(
        Args {
            completion: None,
            cmd: Some(Command::IMAGE {name: "sophon".to_string()}),
        },
        Args::parse_from(&["kbl", "image", "sophon"])
    );
    assert_eq!(
        Args {
            completion: None,
            cmd: Some(Command::DESCRIBE {name: "sophon".to_string()}),
        },
        Args::parse_from(&["kbl", "describe", "sophon"])
    );
    assert_eq!(
        Args {
            completion: None,
            cmd: Some(Command::CONTAINER {name: "sophon".to_string()}),
        },
        Args::parse_from(&["kbl", "container", "sophon"])
    );
}