pub enum Paths {
    Root,
    Data,
    Config,
    Log,
    Exchange
}
impl Paths {
    pub fn get(&self) -> String {
        let root_path = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        match self {
            Paths::Root => root_path,
            Paths::Data => format!("{root_path}/.udoc"),
            Paths::Config => format!("{root_path}/.udoc/config.json"),
            Paths::Log => format!("{root_path}/log.md"),
            Paths::Exchange => format!("{root_path}/.udoc/exchange/"),
        }
    }
}
