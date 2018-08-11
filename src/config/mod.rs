extern crate preferences;

use self::preferences::{AppInfo, Preferences, PreferencesMap};

pub const API_USER: &str = "api_user";
pub const API_KEY: &str = "api_key";

const CONFIG_FILE: &str = "habitica_config";
const APP_INFO: AppInfo = AppInfo {
    name: "Habitica Cli",
    author: "Dawson Freitas Israel",
};

pub fn set_config(api_user: &str, api_key: &str) {
    let mut config = load_config();

    config.insert(API_USER.into(), api_user.into());
    config.insert(API_KEY.into(), api_key.into());

    config.save(&APP_INFO, CONFIG_FILE).unwrap();
}

pub fn read_config(key: &str) -> String {
    let config = load_config();
    config.get(key.into()).unwrap().clone()
}

fn load_config() -> PreferencesMap {
    PreferencesMap::<String>::load(&APP_INFO, CONFIG_FILE)
        .unwrap_or(PreferencesMap::<String>::new())
}
