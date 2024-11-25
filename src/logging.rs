use env_logger::{Builder, Env};

pub fn init_logger() {
    let env = Env::default()
        .filter_or("LOG_LEVEL", "info");
    Builder::from_env(env).init();
}