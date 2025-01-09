pub mod styles {
    use console::Style;
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref GRAY: Style = Style::new().color256(248);
        pub static ref DEBUG: Style = Style::new().magenta();
        pub static ref WARN: Style = Style::new().yellow();
        pub static ref ERROR: Style = Style::new().red();
        pub static ref FATAL: Style = Style::new().red().bright();
    }
}

#[deprecated(since = "0.5.0", note = "apply styles in 'styles' module instead")]
#[macro_export]
macro_rules! set_color {
    (gray) => {
        eprint!("\x1B[38;5;248m");
    };
    (debug) => {
        eprint!("\x1B[35m");
    };
    (warn) => {
        eprint!("\x1B[33m");
    };
    (error) => {
        eprint!("\x1B[31m");
    };
    (fatal) => {
        eprint!("\x1B[91m");
    };
}

#[deprecated(since = "0.5.0", note = "apply styles in 'styles' module instead")]
#[macro_export]
macro_rules! reset_color {
    () => {
        eprint!("\x1B[0m");
    };
}

#[macro_export]
macro_rules! grayln {
    () => {
        eprintln!()
    };
    ($($arg:tt)*) => {{
        eprint!("\x1B[38;5;248m");
        eprintln!($($arg)*);
        eprint!("\x1B[0m");
    }};
}

#[macro_export]
macro_rules! debugln {
    () => {
        eprintln!()
    };
    ($($arg:tt)*) => {{
        eprintln!(
            "{} {}",
            $crate::styles::DEBUG.apply_to("[DEBUG]"),
            $crate::styles::DEBUG.apply_to(format!($($arg)*)),
        );
    }};
}

#[macro_export]
macro_rules! debugln_context {
    () => {
        eprintln!()
    };
    ($($arg:tt)*) => {{
        let prefix = format!("[DEBUG ({}:{}:{})]", file!(), line!(), column!());
        eprintln!(
            "{} {}",
            $crate::styles::DEBUG.apply_to(prefix),
            $crate::styles::DEBUG.apply_to(format!($($arg)*)),
        );
    }};
}

#[macro_export]
macro_rules! warnln {
    () => {
        eprintln!()
    };
    ($($arg:tt)*) => {{
        eprintln!(
            "{} {}",
            $crate::styles::WARN.apply_to("[WARNING]"),
            $crate::styles::WARN.apply_to(format!($($arg)*)),
        );
    }};
}

#[macro_export]
macro_rules! warnln_context {
    () => {
        eprintln!()
    };
    ($($arg:tt)*) => {{
        let prefix = format!("[WARNING ({}:{}:{})]", file!(), line!(), column!());
        eprintln!(
            "{} {}",
            $crate::styles::WARN.apply_to(prefix),
            $crate::styles::WARN.apply_to(format!($($arg)*)),
        );
    }};
}

#[macro_export]
macro_rules! errorln {
    () => {
        eprintln!()
    };
    ($($arg:tt)*) => {{
        eprintln!(
            "{} {}",
            $crate::styles::ERROR.apply_to("[ERROR]"),
            $crate::styles::ERROR.apply_to(format!($($arg)*)),
        );
    }};
}

#[macro_export]
macro_rules! errorln_context {
    () => {
        eprintln!()
    };
    ($($arg:tt)*) => {{
        let prefix = format!("[ERROR ({}:{}:{})]", file!(), line!(), column!());
        eprintln!(
            "{} {}",
            $crate::styles::ERROR.apply_to(prefix),
            $crate::styles::ERROR.apply_to(format!($($arg)*)),
        );
    }};
}

#[macro_export]
macro_rules! fatalln {
    () => {
        eprintln!()
    };
    ($($arg:tt)*) => {{
        eprintln!(
            "{} {}",
            $crate::styles::FATAL.apply_to("[FATAL]"),
            $crate::styles::FATAL.apply_to(format!($($arg)*)),
        );
    }};
}

#[macro_export]
macro_rules! fatalln_context {
    () => {
        eprintln!()
    };
    ($($arg:tt)*) => {{
        let prefix = format!("[FATAL ({}:{}:{})]", file!(), line!(), column!());
        eprintln!(
            "{} {}",
            $crate::styles::FATAL.apply_to(prefix),
            $crate::styles::FATAL.apply_to(format!($($arg)*)),
        );
    }};
}
