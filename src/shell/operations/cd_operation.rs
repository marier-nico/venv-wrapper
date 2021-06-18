use crate::shell::shell_operation::ShellOperation;

pub struct CdOperation {
    target_dir: String,
}

impl CdOperation {
    pub fn _to(dir: String) -> Self {
        CdOperation {
            target_dir: dir,
        }
    }

    fn get_eval(&self) -> String {
        format!("cd {}", self.target_dir)
    }
}

impl ShellOperation for CdOperation {
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
