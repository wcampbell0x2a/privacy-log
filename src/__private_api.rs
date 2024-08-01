pub type Value<'a> = &'a str;

fn log_impl(
    args: std::fmt::Arguments,
    level: log::Level,
    &(target, module_path): &(&str, &'static str),
) {
    let mut builder = log::Record::builder();

    builder.args(args).level(level).target(target).module_path_static(Some(module_path));

    log::logger().log(&builder.build());
}

pub fn log<'a>(
    args: std::fmt::Arguments,
    level: log::Level,
    target_module_path_and_loc: &(&str, &'static str),
) {
    log_impl(args, level, target_module_path_and_loc)
}
