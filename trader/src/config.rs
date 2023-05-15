use ini::Ini;

fn get_config() -> Option<Ini> {
    let result = Ini::load_from_file("./config/config.ini");
    match result {
        Ok(ini) => Some(ini),
        Err(_) => None,
    }
}
pub fn get_config_key(section: &str, key: &str) -> String {
    let conf = get_config();

    if let Some(conf) = conf {
        let section = conf.section(Some(section)).unwrap();
        let value = section.get(key).unwrap();
        value.to_string()
    } else {
        println!("Config file not found");
        String::from("")
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
