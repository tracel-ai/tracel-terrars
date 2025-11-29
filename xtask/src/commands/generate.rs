use std::path::PathBuf;

use tracel_xtask::prelude::*;

#[derive(clap::Args)]
pub struct GenerateCmdArgs {
    /// The provider to generate the bindings from.
    #[arg(value_enum)]
    pub prodiver: Provider,
    /// If set then choose a JSON crafted for testing purpose.
    #[arg(short, long)]
    pub test: bool,
}

#[derive(Copy, Clone, Debug, clap::ValueEnum)]
pub enum Provider {
    /// Hashicorp AWS Provider
    Aws,
}

impl Provider {
    pub fn path(&self, test: bool) -> PathBuf {
        match self {
            Provider::Aws => {
                if test {
                    PathBuf::from("crates/provider-hashicorp-aws/terrars-test.json")
                } else {
                    PathBuf::from("crates/provider-hashicorp-aws/terrars.json")
                }
            }
        }
    }
}

pub fn handle_command(args: GenerateCmdArgs) -> anyhow::Result<()> {
    let json = git::git_repo_root_or_cwd()
        .unwrap()
        .join(args.prodiver.path(args.test));
    run_process(
        "cargo",
        &["generate", &json.to_string_lossy()],
        None,
        None,
        "",
    )
}
