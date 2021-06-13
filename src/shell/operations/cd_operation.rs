use crate::shell::shell_operation::ShellOperation;

pub struct CdOperation<'a> {
    target_dir: &'a str,
}

impl<'a> CdOperation<'a> {
    pub fn to(dir: &'a str) -> Self {
        CdOperation {
            target_dir: dir,
        }
    }

    fn get_eval(&self) -> String {
        format!("cd {}", self.target_dir)
    }
}

impl<'a> ShellOperation for CdOperation<'a> {
    fn bash_eval(&self) -> String {
        self.get_eval()
    }

    fn fish_eval(&self) -> String {
        self.get_eval()
    }

    fn zsh_eval(&self) -> String {
        self.get_eval()
    }
}
