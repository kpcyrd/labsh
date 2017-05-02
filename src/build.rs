use std::io;
use std::process::{Command, ExitStatus};

#[derive(Debug)]
pub struct ExecResult {
}

#[derive(Debug)]
pub enum ExecError {
    ExitStatus(ExitStatus),
    Io(io::Error),
}

impl From<io::Error> for ExecError {
    fn from(err: io::Error) -> ExecError {
        ExecError::Io(err)
    }
}


#[derive(Debug)]
pub struct Task {
    prog: String,
    args: Vec<String>,
}

impl Task {
    pub fn new(prog: &str, args: Vec<&str>) -> Task {
        let args = args.into_iter()
            .map(|x| x.to_string())
            .collect();

        Task {
            prog: prog.to_string(),
            args: args,
        }
    }

    pub fn run(&self, dry: bool) -> Result<ExecResult, ExecError> {
        if dry {
            return Ok(ExecResult {});
        }

        let status = try!(Command::new(&self.prog)
                        .args(&self.args)
                        .status());

        if status.success() {
            Ok(ExecResult {})
        } else {
            Err(ExecError::ExitStatus(status))
        }
    }

    pub fn format(&self) -> String {
        let args: Vec<String> = self.args.iter()
            .map(|x| format!("{:?}", x))
            .collect();
        format!("{:?} {}", self.prog, args.join(" "))
    }
}
