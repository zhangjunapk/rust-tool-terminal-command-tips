use crate::config;
use yaml_rust::Yaml;

pub struct Command {
    pub command: String,
    pub args: Vec<Arg>,
    pub description: String,
}

impl Command {
    pub fn load_commands() -> Vec<Command> {
        let mut commands: Vec<Command> = Vec::new();
        parse_commands_new(&mut commands, config::load_yaml());
        commands
    }
}

impl Default for Command {
    fn default() -> Self {
        Command {
            command: String::new(),
            args: Vec::new(),
            description: String::new(),
        }
    }
}

pub struct Arg {
    pub name: String,
    pub description: String,
}

impl Default for Arg {
    fn default() -> Self {
        Arg {
            name: String::new(),
            description: String::new(),
        }
    }
}



fn parse_commands_new(commands: &mut Vec<Command>, vec_yaml: Vec<Yaml>) {
    if let Some(config) = vec_yaml.first().unwrap().as_hash() {
        if let Some(config_vec) = config.get(&Yaml::from_str("config")).unwrap().as_hash() {
            if let Some(commands_arr) = config_vec
                .get(&Yaml::from_str("commands"))
                .unwrap()
                .as_vec()
            {
                for command_yaml in commands_arr {
                    if let Some(command_hash) = command_yaml.as_hash() {
                        let command_name = command_hash
                            .get(&Yaml::from_str("command"))
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string();
                        let command_description = command_hash
                            .get(&Yaml::from_str("description"))
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string();
                        let args_array = command_hash
                            .get(&Yaml::from_str("args"))
                            .unwrap()
                            .as_vec()
                            .unwrap();
                        let mut args = Vec::new();
                        args_array.iter().for_each(|arg| {
                            if let Some(arg_hash) = arg.as_hash() {
                                let arg_name = arg_hash
                                    .get(&Yaml::from_str("name"))
                                    .unwrap()
                                    .as_str()
                                    .unwrap()
                                    .to_string();
                                let arg_description = arg_hash
                                    .get(&Yaml::from_str("description"))
                                    .unwrap()
                                    .as_str()
                                    .unwrap()
                                    .to_string();
                                let arg = Arg {
                                    name: arg_name,
                                    description: arg_description,
                                };
                                args.push(arg);
                            }
                        });
                        let mut command = Command::default();
                        command.command = command_name;
                        command.description = command_description;
                        command.args = args;
                        commands.push(command);
                    }
                }
            }
        }
    }
}

#[deprecated]
fn parse_commands(commands: &mut Vec<Command>, vec_yaml: Vec<Yaml>) {
    if let Yaml::Hash(commands_hash) = vec_yaml[0].clone() {
        if let Some(config) = commands_hash.get(&Yaml::String("config".to_string())) {
            if let Yaml::Hash(commands_hash) = config.clone() {
                if let Some(commands_yaml) =
                    commands_hash.get(&Yaml::String("commands".to_string()))
                {
                    if let Yaml::Array(array_commands) = commands_yaml {
                        for yaml in array_commands {
                            match yaml {
                                Yaml::Hash(hash) => {
                                    if let Some(command_some) =
                                        hash.get(&Yaml::String("command".to_string()))
                                    {
                                        if let Yaml::String(command_str) = command_some.clone() {
                                            let mut command = Command::default();
                                            command.command = command_str;
                                            if let Some(args_some) =
                                                hash.get(&Yaml::String("args".to_string()))
                                            {
                                                if let Yaml::Array(args_array) = args_some.clone() {
                                                    args_array.iter().for_each(|item| {
                                                        if let Yaml::Hash(arg_hash) = item {
                                                            let mut arg = Arg::default();
                                                            if let Some(description) =
                                                                arg_hash.get(&Yaml::String(
                                                                    "description".to_string(),
                                                                ))
                                                            {
                                                                if let Yaml::String(
                                                                    description_str,
                                                                ) = description.clone()
                                                                {
                                                                    arg.description =
                                                                        description_str;
                                                                }
                                                            }

                                                            if let Some(name) = arg_hash.get(
                                                                &Yaml::String("name".to_string()),
                                                            ) {
                                                                if let Yaml::String(name) =
                                                                    name.clone()
                                                                {
                                                                    arg.name = name;
                                                                }
                                                            }
                                                            command.args.push(arg);
                                                        }
                                                    })
                                                }
                                            }

                                            if let Some(description_some) =
                                                hash.get(&Yaml::String("description".to_string()))
                                            {
                                                if let Yaml::String(description_str) =
                                                    description_some.clone()
                                                {
                                                    command.description = description_str;
                                                }
                                            }

                                            commands.push(command);
                                        }
                                    }
                                }
                                Yaml::Array(arr) => {
                                    parse_commands(commands, arr.to_vec());
                                }
                                other => {
                                    println!("yaml:{:?}", other);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
