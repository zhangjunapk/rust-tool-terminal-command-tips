use rust_tool_terminal_command_tips::command::Command;

#[test]
fn test_parse() {
    let commands = Command::load_commands();
    commands.iter().for_each(|command| {
        println!("命令:{}  描述:{}", command.command, command.description);
        for x in &command.args {
            println!(" -参数:{},参数描述:{}", x.name, x.description);
        }
    })
}
