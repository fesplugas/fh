use clap::{Parser, Subcommand};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use serde::Deserialize;
use std::process::ExitCode;

use crate::cli::cmd::FlakeHubClient;

use super::CommandExecute;

/// Searches FlakeHub for flakes that match your query.
#[derive(Parser)]
pub(crate) struct ListSubcommand {
    #[command(subcommand)]
    cmd: Subcommands,
}

#[derive(Subcommand)]
enum Subcommands {
    Flakes,
    Orgs,
}

#[async_trait::async_trait]
impl CommandExecute for ListSubcommand {
    async fn execute(self, host: &str) -> color_eyre::Result<ExitCode> {
        use Subcommands::*;

        let client = FlakeHubClient::new(host)?;

        match self.cmd {
            Flakes => {
                let pb = ProgressBar::new_spinner();
                pb.set_style(ProgressStyle::default_spinner());
                match client.flakes().await {
                    Ok(flakes) => {
                        if flakes.is_empty() {
                            println!("No results");
                        } else {
                            for Flake { org, project } in flakes {
                                println!(
                                    "{}{}{}\n    https://flakehub.com/flake/{}/{}",
                                    style(org.clone()).cyan(),
                                    style("/").white(),
                                    style(project.clone()).red(),
                                    style(org).cyan(),
                                    style(project).red(),
                                );
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error: {e}");
                    }
                }
            }
            Orgs => {
                let pb = ProgressBar::new_spinner();
                pb.set_style(ProgressStyle::default_spinner());
                match client.orgs().await {
                    Ok(orgs) => {
                        if orgs.is_empty() {
                            println!("No results");
                        } else {
                            for org in orgs {
                                println!(
                                    "{}\n    https://flakehub.com/org/{}",
                                    style(org.clone()).cyan(),
                                    style(org).cyan(),
                                );
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error: {e}");
                    }
                }
            }
        }

        Ok(ExitCode::SUCCESS)
    }
}

#[derive(Deserialize)]
pub(super) struct Flake {
    org: String,
    project: String,
}

#[derive(Deserialize)]
pub(super) struct Org {
    pub(super) name: String,
}
