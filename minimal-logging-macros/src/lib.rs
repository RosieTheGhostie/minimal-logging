#[macro_export]
macro_rules! debugln {
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
        eprint!("\x1B[91m[FATAL ({}:{}:{})] ", file!(), line!(), column!());
        eprintln!($($arg)*);
        eprint!("\x1B[0m");
    }};
}
