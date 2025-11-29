use tracel_xtask::prelude::*;

#[macros::base_commands(Build, Bump, Check, Fix, Infra, Test, Publish)]
enum Command {}

fn main() -> anyhow::Result<()> {
    let (args, environment) = init_xtask::<Command>(parse_args::<Command>()?)?;
    dispatch_base_commands(args, environment)?;
    Ok(())
}
