use std::{convert::TryFrom, path::Path};

use virtualenv::{
    create_virtualenv, file_adder::get_file_adder, python_adder::get_python_adder,
    python_version::PythonVersion, venv_directory_creator::get_directory_creator,
};

mod cli;
mod virtualenv;

fn main() {
    let directory_creator = get_directory_creator().unwrap();
    let file_adder = get_file_adder();
    let interpreter = Path::new("/usr/bin/python3.9");
    let python_adder = get_python_adder().unwrap();
    let python_version = PythonVersion::try_from("3.9.5").unwrap();
    let venv_root = Path::new("/home/nmarier/Documents/Software/Projects/venv-wrapper/venv");

    let result = create_virtualenv(
        &*directory_creator,
        &*file_adder,
        interpreter,
        &*python_adder,
        &python_version,
        venv_root,
    );
    if let Err(e) = result {
        print!("{:?}", e);
    }
}
