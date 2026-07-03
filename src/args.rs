use clap::{
    builder::{BoolishValueParser, NonEmptyStringValueParser, TypedValueParser},
    Arg, ArgAction, Command,
};

use crate::user::User;

pub struct Args {
    init_map: bool,
    save_file: String,
    map_gen_settings: String,
    map_settings: String,
    user: Option<User>,
    exec: Vec<String>,
}

impl Args {
    pub fn new() -> Self {
        let args = Command::new("factorio-up")
            .arg(
                Arg::new("init_map")
                    .long("init-map")
                    .help("Initialize the map settings")
                    .default_value("false")
                    .value_parser(BoolishValueParser::new()),
            )
            .arg(
                Arg::new("save_file")
                    .long("save-file")
                    .help("File path to the save .zip")
                    .default_value("server-default.zip")
                    .value_parser(NonEmptyStringValueParser::new()),
            )
            .arg(
                Arg::new("map_gen_settings")
                    .long("map-gen-settings")
                    .help("File path to the map generator settings")
                    .default_value("map-gen-settings.json")
                    .value_parser(NonEmptyStringValueParser::new()),
            )
            .arg(
                Arg::new("map_settings")
                    .long("map-settings")
                    .help("File path to the map settings")
                    .default_value("map-settings.json")
                    .value_parser(NonEmptyStringValueParser::new()),
            )
            .arg(
                Arg::new("user")
                    .long("user")
                    .help("Run the command as this user")
                    .value_parser(
                        NonEmptyStringValueParser::new()
                            .try_map(|name| User::from_name(name.as_str())),
                    ),
            )
            .arg(
                Arg::new("exec")
                    .hide(true)
                    .trailing_var_arg(true)
                    .num_args(1..)
                    .action(ArgAction::Append)
                    .value_parser(NonEmptyStringValueParser::new()),
            )
            .get_matches();

        Self {
            init_map: *args.get_one("init_map").unwrap(),
            save_file: args.get_one::<String>("save_file").unwrap().to_string(),
            map_gen_settings: args
                .get_one::<String>("map_gen_settings")
                .unwrap()
                .to_string(),
            map_settings: args.get_one::<String>("map_settings").unwrap().to_string(),
            user: args.get_one::<User>("user").cloned(),
            exec: args
                .get_many("exec")
                .unwrap_or_default()
                .map(|s: &String| s.to_string())
                .collect(),
        }
    }

    pub fn init_map(&self) -> bool {
        self.init_map
    }

    pub fn save_file(&self) -> &str {
        &self.save_file
    }

    pub fn map_gen_settings(&self) -> &str {
        &self.map_gen_settings
    }

    pub fn map_settings(&self) -> &str {
        &self.map_settings
    }

    pub fn user(&self) -> Option<User> {
        self.user.clone()
    }

    pub fn exec(&self) -> Vec<&str> {
        self.exec.iter().map(|s| s.as_str()).collect()
    }

    pub fn has_exec(&self) -> bool {
        !self.exec.is_empty()
    }
}
