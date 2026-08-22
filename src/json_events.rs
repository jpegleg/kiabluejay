pub use log::Level;
use log::{LevelFilter, Record, kv, logger};
use std::fmt;
pub extern crate env_logger;
use env_logger::Builder;
use std::{
    io::{self, Write},
    panic, thread,
};

pub const STATIC_MAX_LEVEL: LevelFilter = log::STATIC_MAX_LEVEL;

#[inline]
pub fn max_level() -> LevelFilter {
    log::max_level()
}

#[macro_export(local_inner_macros)]
macro_rules! log {
    (target: $target:expr, $lvl:expr, $e:expr) => {
        $crate::log_impl!(target: $target, $lvl, ($e));
    };
    (target: $target:expr, $lvl:expr, $e:expr, $($rest:tt)*) => {
        $crate::log_impl!(target: $target, $lvl, ($e) $($rest)*);
    };
    ($lvl:expr, $($arg:tt)+) => ($crate::log!(target: __log_module_path!(), $lvl, $($arg)+))
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! log_impl {
    (target: $target:expr, $lvl:expr, ($($arg:expr),*)) => {{
        let lvl = $lvl;
        if lvl <= $crate::STATIC_MAX_LEVEL && lvl <= $crate::max_level() {
            $crate::__private_api_log(
                __log_format_args!($($arg),*),
                lvl,
                &($target, __log_module_path!(), __log_file!(), __log_line!()),
                None,
            );
        }
    }};

    (target: $target:expr, $lvl:expr, ($($arg:expr),*) { $($key:ident : $value:expr),* }) => {{
        if $lvl <= STATIC_MAX_LEVEL && $lvl <= max_level() {
            __private_api_log(
                __log_format_args!($($arg),*),
                $lvl,
                &(__log_module_path!(), __log_module_path!(), __log_file!(), __log_line!()),
                Some(&[$((__log_stringify!($key), &$value)),*])
            );
        }
    }};

    (target: $target:expr, $lvl:expr, ($($e:expr),*) { $($key:ident : $value:expr,)* }) => {
        $crate::log_impl!(target: $target, $lvl, ($($e),*) { $($key : $value),* });
    };

    (target: $target:expr, $lvl:expr, ($($e:expr),*) $arg:expr) => {
        $crate::log_impl!(target: $target, $lvl, ($($e,)* $arg));
    };

    (target: $target:expr, $lvl:expr, ($($e:expr),*) $arg:expr, $($rest:tt)*) => {
        $crate::log_impl!(target: $target, $lvl, ($($e,)* $arg) $($rest)*);
    };
}

#[macro_export(local_inner_macros)]
macro_rules! trace {
    (target: $target:expr, $($arg:tt)+) => (
        log!(target: $target, Level::Trace, $($arg)+);
    );
    ($($arg:tt)+) => (
        log!(Level::Trace, $($arg)+);
    )
}

#[macro_export(local_inner_macros)]
macro_rules! debug {
    (target: $target:expr, $($arg:tt)+) => (
        log!(target: $target, $crate::Level::Debug, $($arg)+);
    );
    ($($arg:tt)+) => (
        log!($crate::Level::Debug, $($arg)+);
    )
}

#[macro_export(local_inner_macros)]
macro_rules! info {
    (target: $target:expr, $($arg:tt)+) => (
        log!(target: $target, $crate::Level::Info, $($arg)+);
    );
    ($($arg:tt)+) => (
        log!($crate::Level::Info, $($arg)+);
    )
}

#[macro_export(local_inner_macros)]
macro_rules! warn {
    (target: $target:expr, $($arg:tt)+) => (
        log!(target: $target, $crate::Level::Warn, $($arg)+);
    );
    ($($arg:tt)+) => (
        log!($crate::Level::Warn, $($arg)+);
    )
}

#[macro_export(local_inner_macros)]
macro_rules! error {
    (target: $target:expr, $($arg:tt)+) => (
        log!(target: $target, $crate::Level::Error, $($arg)+);
    );
    ($($arg:tt)+) => (
        log!(Level::Error, $($arg)+);
    )
}

#[macro_export(local_inner_macros)]
macro_rules! log_enabled {
    (target: $target:expr, $lvl:expr) => {{
        let lvl = $lvl;
        lvl <= $crate::STATIC_MAX_LEVEL
            && lvl <= $crate::max_level()
            && $crate::__private_api_enabled(lvl, $target)
    }};
    ($lvl:expr) => {
        log_enabled!(target: __log_module_path!(), $lvl)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_format_args {
    ($($args:tt)*) => {
        format_args!($($args)*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_module_path {
    () => {
        module_path!()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_file {
    () => {
        file!()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_line {
    () => {
        line!()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_stringify {
    ($($args:tt)*) => {
        stringify!($($args)*)
    };
}

pub fn __private_api_log(
    args: fmt::Arguments<'_>,
    level: Level,
    &(target, module_path, file, line): &(&str, &'static str, &'static str, u32),
    kvs: Option<&[(&str, &dyn log::kv::ToValue)]>,
) {
    logger().log(
        &Record::builder()
            .args(args)
            .level(level)
            .target(target)
            .module_path_static(Some(module_path))
            .file_static(Some(file))
            .line(Some(line))
            .key_values(&kvs)
            .build(),
    );
}

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
                error!(
                    "panicked at '{}'", errout,
                    {
                        thread: thread,
                        location: format!("{}:{}", location.file(), location.line())
                    }
                );
            }
            None => {
                error!("panicked at '{}'", errout, { thread: thread });
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
