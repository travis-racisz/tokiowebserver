pub struct Config {
    pub database_url: String,
    pub database_port: u16,
}

impl Config {
    pub fn set_env() {
        let vars = std::fs::read_to_string(".env").expect("cannot read env file");
        for line in vars.lines() {
            let mut iter = line.split("=");
            let key = iter.next().unwrap();
            let value = iter.next().unwrap();
            std::env::set_var(key, value);
        }
    }
}
