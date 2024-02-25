use env_logger::{Builder, Env};
use std::{
    env,
    io::{self, Write},
    panic, thread,
};

pub fn init() {
    let format = env::var("MOCKSER_LOG_FORMAT").unwrap_or_else(|_| "human".to_string());
    match format.as_str() {
        "json" => init_json(),
        _ => init_human(),
    }
}

fn new_env() -> Env<'static> {
    Env::new()
        .filter_or("MOCKSER_LOG", "info")
        .write_style("MOCKSER_LOG_STYLE")
}

fn init_human() {
    human_panic::setup_panic!();

    Builder::from_env(new_env())
        .format_timestamp_millis()
        .init();
}

fn init_json() {
    panic_hook();
    Builder::from_env(new_env()).format(write_json).init();
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
