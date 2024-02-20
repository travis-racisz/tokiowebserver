pub struct Config {
    pub surreal_url: String,
}

impl Config {
    pub fn set_env() -> Self {
        let vars = std::fs::read_to_string(".env").expect("cannot read env file");
        for line in vars.lines() {
            let mut iter = line.split("=");
            let key = iter.next().unwrap();
            let value = iter.next().unwrap();
            std::env::set_var(key, value);
        }
        let database_url = std::env::var("SURREAL_URL").expect("DATABASE_URL is not set");
        Config {
            surreal_url: database_url,
        }
    }
}
