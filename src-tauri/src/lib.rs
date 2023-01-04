use serde::{Deserialize, Serialize};
use std::process::Command;
use std::time::Instant;

#[derive(Serialize, Deserialize, Debug)]
pub struct OsCommand {
    command: String,
    args: Vec<String>,
    rc: i32,
    stdout: String,
    stderr: String,
    duration_milliseconds: u128,
    max_executions: u32,

    executions: u32,
    successful_executions: u32,
    successful_executions_with_rc_zero: u32,
    failed_executions: u32,
    rejected_executions: u32,
    total_execution_calls: u32,
}

impl OsCommand {
    pub fn new(command: &str, args: Vec<String>) -> OsCommand {
        OsCommand {
            command: command.to_string(),
            args: args.clone(),
            rc: -1,
            stdout: String::from("Command not yet executed"),
            stderr: String::from("Command not yet executed"),
            duration_milliseconds: 0,
            max_executions: std::u32::MAX,
            executions: 0,
            successful_executions: 0,
            successful_executions_with_rc_zero: 0,
            failed_executions: 0,
            rejected_executions: 0,
            total_execution_calls: 0,
        }
    }

    pub fn parse(command_line: &str) -> OsCommand {
        let parts: Vec<&str> = command_line.split(" ").collect();

        let command = parts[0];

        if parts.len() > 1 {
            let args = parts[1..].iter().map(|&s| s.into()).collect();
            return OsCommand::new(command, args);
        } else {
            let args: Vec<String> = vec![];
            return OsCommand::new(command, args);
        }
    }

    pub fn set_command(&mut self, command_line: &str) {
        let parts: Vec<&str> = command_line.split(" ").collect();

        let command = parts[0];
        self.command = command.to_string();

        if parts.len() > 1 {
            let args = parts[1..].iter().map(|&s| s.into()).collect();
            self.args = args;
        } else {
            let args: Vec<String> = vec![];
            self.args = args;
        }
    }

    pub fn create_onetime_command(command_line: &str) -> OsCommand {
        let mut cmd = OsCommand::parse(command_line);
        cmd.set_max_executions(1);

        cmd
    }

    pub fn set_max_executions(&mut self, max_executions: u32) {
        self.max_executions = max_executions;
    }

    pub fn execute(&mut self) {
        self.total_execution_calls += 1;

        if self.executions == self.max_executions {
            self.rejected_executions += 1;
            return;
        }

        let start = Instant::now();
        let o = Command::new(&self.command).args(&self.args).output();

        self.executions += 1;

        match o {
            Ok(output) => {
                let status = output.status;
                match status.code() {
                    Some(rc) => {
                        self.rc = rc;
                        if rc == 0 {
                            self.successful_executions_with_rc_zero += 1;
                        }
                    }
                    None => self.rc = -1,
                }

                self.successful_executions += 1;

                self.stdout = String::from_utf8(output.stdout).unwrap();
                self.stderr = String::from_utf8(output.stderr).unwrap();
            }
            Err(err) => {
                self.rc = -1;
                self.stderr = format!("{:?}", err);
                self.failed_executions += 1;
            }
        }

        self.duration_milliseconds = start.elapsed().as_millis();
    }
}
