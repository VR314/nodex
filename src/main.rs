mod args;
mod node;
mod project;

use clap::Parser;

use std::fs;
use std::io::Write;

use args::{
    EntityType::{Node as NodeEntity, Project as ProjectEntity, Run as RunEntity},
    NodexArgs, *,
};

use project::Project;

fn main() {
    let args = NodexArgs::parse();
    println!("{:?}", args);

    match args.entity_type {
        ProjectEntity(command) => match command.command {
            ProjectSubcommand::New(new_project_args) => {
                println!("Creating new project: {:?}", new_project_args);

                // populate default values
                let mut name = String::from("nodex_project");
                let mut path = std::env::current_dir().unwrap();

                if !new_project_args.yes {
                    if let Some(given_name) = new_project_args.name {
                        name = given_name;
                    }
                    name = get_input(Some("Project name"), Some(&name)).unwrap();

                    path.push(&name); // only update path after name is confirmed
                    path = get_input(Some("Path to project root"), Some(path.to_str().unwrap()))
                        .unwrap()
                        .into();

                    let input = get_input(Some("Is this correct?"), Some("y")).unwrap();
                    if input.trim().to_lowercase() != "y"
                        && input.trim().to_lowercase() != "yes"
                        && !input.trim().is_empty()
                    {
                        println!("Aborting project creation.");
                        return; // end program if user does not confirm
                    }
                }

                // create project directory structure
                fs::create_dir_all(path.join("nodes")).unwrap();
                fs::create_dir_all(path.join("topics")).unwrap();

                // create <project_name>.nodex file
                fs::File::create(path.join(format!("{name}.nodex"))).unwrap();
            }
        },
        NodeEntity(command) => match command.command {
            NodeSubcommand::New(new_node_args) => {
                println!("Creating new node: {:?}", new_node_args);
            }
        },
        RunEntity(command) => {
            println!("Running nodes: {:?}", command.nodes);
        }
    }
}

fn get_input(prompt: Option<&str>, default_val: Option<&str>) -> Result<String, std::io::Error> {
    if let Some(p) = prompt {
        if let Some(d) = default_val {
            print!("{p} ({d}): ");
        } else {
            print!("{p}: ");
        }
    }
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            if let Some(d) = default_val {
                if input.trim().is_empty() {
                    // if default value supplied and input is empty, return default value
                    return Ok(String::from(d));
                }
            }
            Ok(input)
        }
        Err(error) => Err(error),
    }
}
