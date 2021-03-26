use log::info;

pub fn service_routine() {
    info!(
        "Executing service routine: {}",
        chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]")
    );
}
