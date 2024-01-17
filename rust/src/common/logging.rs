
#[macro_export]
macro_rules! print_red {
    ($($arg:tt)*) => {
        use colored::*;
        info!("{}", format!($($arg)*).red());
    };
}
#[macro_export]
macro_rules! print_green {
    ($($arg:tt)*) => {
        use colored::*;
        info!("{}", format!($($arg)*).green());
    };
}
#[macro_export]
macro_rules! print_yellow {
    ($($arg:tt)*) => {
        use colored::*;
        info!("{}", format!($($arg)*).yellow());
    };
}
#[macro_export]
macro_rules! print_blue {
    ($($arg:tt)*) => {
        use colored::*;
        info!("{}", format!($($arg)*).blue());
    };
}