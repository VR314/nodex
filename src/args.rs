use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct NodexArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    Project(ProjectCommand),
    Node(NodeCommand),
    Run(RunCommand),
}

#[derive(Debug, Args)]
pub struct ProjectCommand {
    #[clap(subcommand)]
    pub command: ProjectSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum ProjectSubcommand {
    New(NewProjectArgs),
}

#[derive(Debug, Args)]
pub struct NewProjectArgs {
    /// Sets the name of the project, and the directory it will be created in
    pub name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Flag to skip confirmation prompt, and use all default values
    #[arg(short, long)]
    pub yes: bool,
}

#[derive(Debug, Args)]
pub struct NodeCommand {
    #[clap(subcommand)]
    pub command: NodeSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum NodeSubcommand {
    New(NewNodeArgs),
}

#[derive(Debug, Args)]
pub struct NewNodeArgs {
    pub name: String,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,
}

#[derive(Debug, Args)]

pub struct RunCommand {
    pub nodes: Vec<String>,
}
