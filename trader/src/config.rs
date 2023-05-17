use ini::Ini;
use std::fs;

fn get_config() -> Option<Ini> {
    fs::create_dir_all("./config/").ok()?;
    let result = Ini::load_from_file("./config/config.ini");

    match result {
        Ok(ini) => Some(ini),
        Err(_) => {
            // config was not found so lets create it
            let create_result = fs::File::create("./config/config.ini");
            match create_result {
                Ok(_) => {
                    let ini = Ini::new();
                    drop(ini.write_to_file("./config/config.ini"));
                    Some(ini)
                }
                Err(_) => {
                    None
                    //println!("Error creating config file");
                }
            }
        }
    }
}
pub fn get_config_key(section: &str, key: &str) -> Option<String> {
    let conf = get_config();

    if let Some(conf) = conf {
        let section = conf.section(Some(section));
        match section {
            Some(section) => {
                let value = section.get(key);
                match value {
                    Some(value) => {
                        Some(value.to_string())
                        //println!("{}", value);
                    }
                    None => {
                        println!("Key not found");
                        None
                    }
                }
            }
            None => {
                println!("Section not found");
                None
            }
        }
    } else {
        println!("Config file not found");
        None
    }
}
pub fn set_config_key(section: &str, key: &str, value: &str) {
    let conf = get_config();
    if let Some(mut conf) = conf {
        let mut section = conf.with_section(Some(section));
        section.set(key, value);
        conf.write_to_file("./config/config.ini").unwrap();
    } else {
        println!("Config file not found");
    }
}
