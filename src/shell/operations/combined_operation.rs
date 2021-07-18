use crate::{shell::shell_operation::ShellOperation, utils::intersperse};

pub struct CombinedOperation {
    operations: Vec<Box<dyn ShellOperation>>,
}

impl CombinedOperation {
    pub fn new(operations: Vec<Box<dyn ShellOperation>>) -> Self {
        CombinedOperation {
            operations,
        }
    }
}

impl ShellOperation for CombinedOperation {
    fn bash_eval(&self) -> String {
        let evals = self.operations.iter().map(|e| e.bash_eval());

        intersperse(evals, '\n')
    }

    fn fish_eval(&self) -> String {
        let evals = self.operations.iter().map(|e| e.fish_eval());

        intersperse(evals, '\n')
    }

    fn zsh_eval(&self) -> String {
        let evals = self.operations.iter().map(|e| e.zsh_eval());

        intersperse(evals, '\n')
    }
}
