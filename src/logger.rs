use env_logger::{Builder, Env};
use std::{
    env,
    io::{self, Write},
    panic, thread,
};

pub fn init(prefix: &str) {
    let prefix = prefix.to_uppercase();
    let log_format_env = format!("{}_LOG_FORMAT", prefix);
    let format = env::var(log_format_env).unwrap_or_else(|_| "human".to_string());
    match format.as_str() {
        "json" => init_json(prefix.as_str()),
        _ => init_human(prefix.as_str()),
    }
}

fn new_env(prefix: &str) -> Env<'static> {
    let filter_env = format!("{}_LOG", prefix);
    let style_env = format!("{}_LOG_STYLE", prefix);
    Env::new()
        .filter_or(filter_env, "info")
        .write_style(style_env)
}

fn init_human(prefix: &str) {
    human_panic::setup_panic!();

    Builder::from_env(new_env(prefix))
        .format_timestamp_millis()
        .init();
}

fn init_json(prefix: &str) {
    panic_hook();
    Builder::from_env(new_env(prefix)).format(write_json).init();
}

fn write_json<F>(f: &mut F, record: &log::Record) -> io::Result<()>
where
    F: Write,
{
    write!(f, "{{")?;
    write!(f, "\"level\":\"{}\",", record.level())?;
    write!(
        f,
        "\"ts\":{}",
        std::time::UNIX_EPOCH.elapsed().unwrap().as_millis()
    )?;
    write!(f, ",\"msg\":")?;
    write_json_str(f, &record.args().to_string())?;
    writeln!(f, "}}")
}

fn write_json_str<W: io::Write>(writer: &mut W, raw: &str) -> std::io::Result<()> {
    serde_json::to_writer(writer, raw)?;
    Ok(())
}

pub fn panic_hook() {
    panic::set_hook(Box::new(|info| {
        let thread = thread::current();
        let thread = thread.name().unwrap_or("unnamed");

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &**s,
                None => "Box<Any>",
            },
        };
        match info.location() {
            Some(location) => {
                log::error!(
                    "{} panicked at thread: {} and location {}:{}",
                    msg,
                    thread,
                    location.file(),
                    location.line()
                );
            }
            None => {
                log::error!(
                    "{} panicked at thread: {} and unknown location",
                    thread,
                    msg
                );
            }
        }
    }));
}
