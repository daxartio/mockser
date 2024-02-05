use env_logger::{Builder, Env};

pub fn init() {
    let env = Env::new()
        .filter_or("MOCKSER_LOG", "info")
        .write_style("MOCKSER_LOG_STYLE");
    Builder::from_env(env).init();
}
