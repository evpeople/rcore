use crate::sbi::console_putchar;
use core::fmt::{self, Write};
use log::{self, Level, LevelFilter, Log, Metadata, Record};
struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize)
        }
        Ok(())
    }
}
pub fn init() {
    static LOGGER: SimpleLogger = SimpleLogger;
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some("error") => {
            print(format_args!("Error"));
            LevelFilter::Error
        }
        Some("warn") => {
            print(format_args!("Warn"));
            LevelFilter::Warn
        }
        Some("info") => {
            print(format_args!("Info"));
            LevelFilter::Info
        }
        Some("debug") => {
            print(format_args!("Debug"));
            LevelFilter::Debug
        }
        Some("trace") => {
            print(format_args!("Trace"));
            LevelFilter::Trace
        }
        _ => {
            print(format_args!("d4d"));
            LevelFilter::Off
        }
    });
}

pub fn print(args: fmt::Arguments) {
    //write_fmt is  impl by Write
    Stdout.write_fmt(args).unwrap();
}
#[macro_export]
macro_rules! print {
	($fmt:literal $(,$($arg:tt)+)?) => {
		$crate::console::print(format_args!($fmt $(,$($arg)+)?));
	};
}
#[macro_export]
macro_rules! println {
	($fmt: literal $(, $($arg: tt)+)?) => {
		$crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
	}
}

struct SimpleLogger;
impl Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        print_in_color(
            format_args!("{}", record.args()),
            level_to_color_code(record.level()),
        );
    }
    fn flush(&self) {}
}
fn level_to_color_code(level: Level) -> u8 {
    match level {
        Level::Error => 31, // Red
        Level::Warn => 93,  // BrightYellow
        Level::Info => 34,  // Blue
        Level::Debug => 32, // Green
        Level::Trace => 90, // BrightBlack
    }
}
macro_rules! with_color {
    ($args: ident, $color_code: ident) => {{
        format_args!("\u{1B}[{}m{}\u{1B}[0m", $color_code as u8, $args)
    }};
}

fn print_in_color(args: fmt::Arguments, color_code: u8) {
    Stdout.write_fmt(with_color!(args, color_code)).unwrap();
}
