use serde_derive::{Serialize, Deserialize};
use std::{env, fs, io::{self, ErrorKind, Error}, path::PathBuf};

#[derive(Deserialize, Serialize, Debug)]
pub struct Instance {
    pub name: String,
    pub version: String,
    kind: GameType,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum GameType {
    Vanilla,
}

pub fn get_instance_dir() -> io::Result<PathBuf> {
    let exe_path = env::current_exe()?;
    let mut path = dunce::canonicalize(exe_path)?;
    path.pop();
    path.push("instances");
    Ok(path)
}

pub fn directory_exist() -> io::Result<bool> {
    get_instance_dir()?.try_exists()
}

pub fn get_all_instances() {}

pub fn get_instance(name: &str) -> io::Result<Instance> {
    let instance_name = name;
    let mut instance_path = get_instance_dir()?;
    instance_path.push(instance_name);
    instance_path.push("instance.toml");
    let instance_content = fs::read_to_string(instance_path)?;
    let instance: Instance = match toml::from_str(&instance_content) {
        Ok(instance) => instance,
        Err(e) => return Err(Error::new(ErrorKind::InvalidData, e.message())),
    };
    Ok(instance)
}

pub fn create_instance(name: &str, version: &str, kind: GameType) -> io::Result<Instance> {
    if !directory_exist()? {
        let instance = get_instance_dir()?;
        fs::create_dir(instance.as_path())?;
    }

    let instance_name = name;
    let mut instance_dir = get_instance_dir()?;
    instance_dir.push(instance_name);
    fs::create_dir(instance_dir)?;

    let instance = Instance {
        name: instance_name.to_owned(),
        version: version.to_owned(),
        kind: kind,
    };

    let instance_toml = toml::to_string(&instance).unwrap();
    let mut instance_config = get_instance_dir()?;
    instance_config.push(instance_name);
    instance_config.push("instance.toml");
    fs::write(instance_config, &instance_toml)?;

    Ok(instance)
}

fn delete_instance(name: &str) {}
