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
        eprint!("\x1B[35m[DEBUG] ");
        eprintln!($($arg)*);
        eprint!("\x1B[0m");
    }};
}

#[macro_export]
macro_rules! debugln_context {
    () => {
        eprintln!()
    };
    ($($arg:tt)*) => {{
        eprint!("\x1B[35m[DEBUG ({}:{}:{})] ", file!(), line!(), column!());
        eprintln!($($arg)*);
        eprint!("\x1B[0m");
    }};
}

#[macro_export]
macro_rules! warnln {
    () => {
        eprintln!()
    };
    ($($arg:tt)*) => {{
        eprint!("\x1B[33m[WARNING] ");
        eprintln!($($arg)*);
        eprint!("\x1B[0m");
    }};
}

#[macro_export]
macro_rules! warnln_context {
    () => {
        eprintln!()
    };
    ($($arg:tt)*) => {{
        eprint!("\x1B[33m[WARNING ({}:{}:{})] ", file!(), line!(), column!());
        eprintln!($($arg)*);
        eprint!("\x1B[0m");
    }};
}

#[macro_export]
macro_rules! errorln {
    () => {
        eprintln!()
    };
    ($($arg:tt)*) => {{
        eprint!("\x1B[31m[ERROR] ");
        eprintln!($($arg)*);
        eprint!("\x1B[0m");
    }};
}

#[macro_export]
macro_rules! errorln_context {
    () => {
        eprintln!()
    };
    ($($arg:tt)*) => {{
        eprint!("\x1B[31m[ERROR ({}:{}:{})] ", file!(), line!(), column!());
        eprintln!($($arg)*);
        eprint!("\x1B[0m");
    }};
}

#[macro_export]
macro_rules! fatalln {
    () => {
        eprintln!()
    };
    ($($arg:tt)*) => {{
        eprint!("\x1B[91m[FATAL] ");
        eprintln!($($arg)*);
        eprint!("\x1B[0m");
    }};
}

#[macro_export]
macro_rules! fatalln_context {
    () => {
        eprintln!()
    };
    ($($arg:tt)*) => {{
        eprint!("\x1B[91m[FATAL ({}:{}:{})] ", file!(), line!(), column!());
        eprintln!($($arg)*);
        eprint!("\x1B[0m");
    }};
}
