pub trait ShellOperation {
    fn bash_eval(&self) -> String;
    fn fish_eval(&self) -> String;
    fn zsh_eval(&self) -> String;
}
