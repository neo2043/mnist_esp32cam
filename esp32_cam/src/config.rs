#[derive(Debug)]
pub struct Config {
    pub wifi_ssid: &'static str,
    pub wifi_psk: &'static str,
}

pub fn get_config() -> Config {
    Config {
        wifi_ssid: "Excitel_2.4g",
        wifi_psk: "awkps5929g",
    }
}
