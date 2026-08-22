pub extern crate env_logger;
use env_logger::Builder;
use log::{LevelFilter, kv};
use std::{
    io::{self, Write},
    panic, thread,
};

pub fn panic_catch() {
    panic::set_hook(Box::new(|info| {
        let thread = thread::current();
        let thread = thread.name().unwrap_or("unnamed");
        let errout = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &**s,
                None => "Box<Any>",
            },
        };
        match info.location() {
            Some(location) => {
                kv_log_macro::error!(
                    "panicked at '{}'", errout,
                    {
                        thread: thread,
                        location: format!("{}:{}", location.file(), location.line())
                    }
                );
            }
            None => {
                kv_log_macro::error!("panicked at '{}'", errout, { thread: thread });
            }
        }
    }));
}

pub fn builder() -> Builder {
    let mut builder = Builder::from_default_env();
    builder.filter_level(LevelFilter::Info).format(write);
    builder
}

fn write<F>(f: &mut F, record: &log::Record) -> io::Result<()>
where
    F: Write,
{
    write!(f, "{{")?;
    write!(f, "\"time\":\"{}\"", chrono::Utc::now().to_rfc3339())?;
    write!(f, ",\"level\":\"{}\"", record.level())?;
    if record.args().to_string().starts_with("\"") {
        let jrecord = &record.args().to_string();
        write!(f, ",{}", jrecord)?;
    } else {
        if record.args().to_string().starts_with("{") {
            let jrecord = &record.args().to_string();
            write!(f, ",\"event\": {}", jrecord)?;
        } else {
            let stripj = &record.args().to_string().replace("\"", "'");
            let jrecord = format!("\"event\":\"{}\"", stripj);
            write!(f, ",{}", jrecord)?;
        }
    }
    struct Visitor<'a, W: Write> {
        writer: &'a mut W,
    }

    impl<'kvs, 'a, W: Write> kv::Visitor<'kvs> for Visitor<'a, W> {
        fn visit_pair(
            &mut self,
            key: kv::Key<'kvs>,
            val: kv::Value<'kvs>,
        ) -> Result<(), kv::Error> {
            write!(self.writer, ",\"{}\":{}", key, val)?;
            Ok(())
        }
    }

    let mut visitor = Visitor { writer: f };
    record.key_values().visit(&mut visitor).unwrap();
    writeln!(f, "}}")
}
