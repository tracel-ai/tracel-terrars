use tracel_xtask::prelude::*;

mod commands;

#[macros::base_commands(Build, Bump, Check, Fix, Infra, Test, Publish)]
enum Command {
    /// Generate provider bindings
    Generate(commands::generate::GenerateCmdArgs),
}

fn main() -> anyhow::Result<()> {
    let (args, environment) = init_xtask::<Command>(parse_args::<Command>()?)?;
    match args.command {
        Command::Generate(cmd_args) => commands::generate::handle_command(cmd_args),
        _ => dispatch_base_commands(args, environment),
    }
}
