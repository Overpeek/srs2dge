#[macro_export]
macro_rules! unwrap_or_return {
    ($opt:expr) => {
        if let Some(__some) = $opt {
            __some
        } else {
            return;
        }
    };

    ($opt:expr, $return:expr) => {
        if let Some(__some) = $opt {
            __some
        } else {
            return $return;
        }
    };
}

// shamelessly stolen from: https://github.com/popzxc/stdext-rs
#[macro_export]
macro_rules! function_name {
    () => {{
        // Okay, this is ugly, I get it. However, this is the best we can get on a stable rust.
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        // `3` is the length of the `::f`.
        &name[..name.len() - 3]
    }};
}

#[macro_export]
macro_rules! label {
    () => {
        Some($crate::function_name!())
    };
}
