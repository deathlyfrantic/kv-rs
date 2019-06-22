use clap::{crate_authors, crate_name, crate_version, App, AppSettings, Arg, SubCommand};
use serde_json::{from_value, to_value, Map, Value};
use std::{
    fs::{read_to_string, write},
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
    process::exit,
};

fn file_path() -> PathBuf {
    let xdg_data_home = Path::new(env!("XDG_DATA_HOME"));
    if xdg_data_home.is_dir() {
        xdg_data_home.join("kv.json")
    } else {
        Path::new(env!("HOME")).join(".kv.json")
    }
}

fn load_json() -> Result<Map<String, Value>, Error> {
    let json = read_to_string(file_path())?;
    let value: Value = serde_json::from_str(&json)?;
    match value {
        Value::Object(m) => Ok(m),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            "Data in file was not an object.",
        )),
    }
}

fn save_json(json: Value) -> Result<(), Error> {
    write(file_path(), format!("{}", json))
}

fn get(key: &str) -> Result<String, Error> {
    let mut json = load_json()?;
    // we're not going to save this json so we can just remove the item. this avoids getting a
    // reference we have to clone.
    match json.remove(key) {
        Some(v) => Ok(from_value::<String>(v)?),
        None => Err(Error::new(
            ErrorKind::NotFound,
            format!("Key \"{}\" not found.", key),
        )),
    }
}

fn set(key: &str, value: &str, force: bool) -> Result<String, Error> {
    let mut json = load_json().unwrap_or_default();
    if json.contains_key(key) && !force {
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            format!(
                "Key \"{}\" already present. (Use --force to overwrite.)",
                key
            ),
        ));
    }
    json.insert(key.to_string(), to_value(value)?);
    save_json(to_value(json)?)?;
    Ok(format!("Key \"{}\" set to value \"{}\".", key, value))
}

fn delete(key: &str) -> Result<String, Error> {
    let mut json = load_json()?;
    if !json.contains_key(key) {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("Key \"{}\" not found.", key),
        ));
    }
    json.remove(key);
    save_json(to_value(json)?)?;
    Ok(format!("Deleted key \"{}\".", key))
}

fn list() -> Result<String, Error> {
    let json = load_json().unwrap_or_default();
    Ok(if json.is_empty() {
        "No keys found.".to_string()
    } else {
        let mut items = vec![];
        for (k, v) in json.into_iter() {
            items.push(format!("{} -> {}", k, from_value::<String>(v)?));
        }
        items.join("\n")
    })
}

fn complete_commands(app: App) -> Result<String, Error> {
    Ok(app
        .p
        .subcommands
        .iter()
        .filter(|sc| !sc.get_name().starts_with("complete") && sc.p.meta.about.is_some())
        .map(|sc| format!("{}:{}", sc.get_name(), sc.p.meta.about.unwrap()))
        .collect::<Vec<String>>()
        .join("\n"))
}

fn complete_keys() -> Result<String, Error> {
    let json = load_json()?;
    let mut items = vec![];
    for (k, v) in json.into_iter() {
        items.push(format!("{}:{}", k, from_value::<String>(v)?));
    }
    Ok(items.join("\n"))
}

fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(
            SubCommand::with_name("delete")
                .about("Deletes key:value pairs.")
                .arg(
                    Arg::with_name("key")
                        .required(true)
                        .help("The key of the key:value pair to delete."),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Gets the value for a given key.")
                .arg(
                    Arg::with_name("key")
                        .required(true)
                        .help("The key of the value to retrieve."),
                ),
        )
        .subcommand(SubCommand::with_name("list").about("Lists all key:value pairs."))
        .subcommand(
            SubCommand::with_name("set")
                .about("Sets a value for a key.")
                .arg(Arg::with_name("key").required(true).help("The key to set."))
                .arg(
                    Arg::with_name("value")
                        .required(true)
                        .help("The value of the key."),
                )
                .arg(
                    Arg::with_name("force")
                        .short("f")
                        .long("force")
                        .help("Overwrite value if key already exists."),
                ),
        )
        .subcommand(SubCommand::with_name("complete-commands").setting(AppSettings::Hidden))
        .subcommand(SubCommand::with_name("complete-keys").setting(AppSettings::Hidden));
    match match app.clone().get_matches().subcommand() {
        ("delete", Some(sub)) => delete(sub.value_of("key").unwrap()),
        ("get", Some(sub)) => get(sub.value_of("key").unwrap()),
        ("list", _) => list(),
        ("set", Some(sub)) => set(
            sub.value_of("key").unwrap(),
            sub.value_of("value").unwrap(),
            sub.is_present("force"),
        ),
        ("complete-commands", _) => complete_commands(app),
        ("complete-keys", _) => complete_keys(),
        _ => Ok(String::new()),
    } {
        Ok(msg) => println!("{}", msg),
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };
}
