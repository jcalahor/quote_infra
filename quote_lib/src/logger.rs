use fast_log::Config as FastLocalConfig;

pub struct FileNameIdentifiers {
    pub time_stamp: String,
    pub random_nbr: u32,
    pub name_suffix: String,
}

pub fn setup_logger(fni: &FileNameIdentifiers) -> Result<(), Box<dyn std::error::Error>> {
    let file_path: String = format!(
        "logs/{}@{}@{}",
        fni.time_stamp.clone(),
        fni.random_nbr,
        fni.name_suffix
    );
    fast_log::init(
        FastLocalConfig::new()
            .file(file_path.as_str())
            .chan_len(Some(10000))
            .level(log::LevelFilter::Info),
    )
    .unwrap();
    Ok(())
}
