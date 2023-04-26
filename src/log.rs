pub fn fern_init() -> Result<(), Box<dyn std::error::Error>> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{0}[{1}][{2}] {3}",
                chrono::Local::now().format("[%Y/%m/%d][%H:%M:%S]"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(if cfg!(debug_assertion) {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .level_for("wgpu_core", if cfg!(debug_assertion) {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Warn
        })
        .chain(std::io::stdout())
        .apply()?;

    Ok(())
}