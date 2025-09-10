mod display;
mod log;
mod mylib;
mod structs;
mod utils;
use zed_extension_api::{
    self as zed, Result, SlashCommand, SlashCommandArgumentCompletion, SlashCommandOutput,
    SlashCommandOutputSection, Worktree,
};

use crate::structs::TODODir;

struct TestExtension {
    todo_tree: structs::TODODir,
}

impl TestExtension {}

impl zed::Extension for TestExtension {
    fn new() -> Self {
        println!("[TODOTree]Extension Started!");

        Self {
            todo_tree: TODODir {
                name: "Project".to_string(),
                files: vec![],
                dirs: vec![],
            },
        }
    }
    fn complete_slash_command_argument(
        &self,
        command: SlashCommand,
        _args: Vec<String>,
    ) -> Result<Vec<zed_extension_api::SlashCommandArgumentCompletion>, String> {
        println!("[TODOTree]command:{}", command.name);
        match command.name.as_str() {
            "open log" => Ok(vec![SlashCommandArgumentCompletion {
                label: "Option One".to_string(),
                new_text: "option-1".to_string(),
                run_command: true,
            }]),
            "echo" => Ok(vec![]),
            "pick-one" => Ok(vec![
                SlashCommandArgumentCompletion {
                    label: "Option One".to_string(),
                    new_text: "option-1".to_string(),
                    run_command: true,
                },
                SlashCommandArgumentCompletion {
                    label: "Option Two".to_string(),
                    new_text: "option-2".to_string(),
                    run_command: true,
                },
                SlashCommandArgumentCompletion {
                    label: "Option Three".to_string(),
                    new_text: "option-3".to_string(),
                    run_command: true,
                },
            ]),
            command => Err(format!("unknown slash command: \"{command}\"")),
        }
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        _worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        println!("[TODOTree]command:{}", command.name);
        if let Some(worktree) = _worktree {
            println!("root path:{}", worktree.root_path())
        }
        match command.name.as_str() {
            "refresh_todo" => {
                display::display_directory(&self.todo_tree, &0);
                return Ok(SlashCommandOutput {
                    text: "2".to_string(),
                    sections: vec![],
                });
            }
            command => Err(format!("unknown slash command: \"{command}\"")),
        }
    }
}

zed::register_extension!(TestExtension);
