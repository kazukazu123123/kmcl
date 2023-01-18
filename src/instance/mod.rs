use std::{env, io::{Error, ErrorKind, self}};

pub fn directory_exist() -> bool {
    let current_path = env::current_dir();

    match current_path {
        Ok(mut path) => {
            path.push("instances");
            path.exists()
        }
        Err(_) => false,
    }
}

pub fn get_all_instances() {
  
}

pub fn get_instance(name: &str) -> &str {
    name
}

fn create_instance(name: &str, version: &str) -> io::Result<()> {
    match directory_exist() {
        true => {
            println!("name: {}, version: {}", name, version);
            Ok(())
        },
        false => Err(Error::new(ErrorKind::NotFound, "instance directory not found."))
    }
}

fn delete_instance() {}
