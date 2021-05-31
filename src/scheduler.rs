use crate::service;
pub use clokwerk::ScheduleHandle;
use clokwerk::{Interval, Scheduler};
use log::{info, warn};

pub fn launch_fixed_routine(disable: bool, interval: u8) -> Option<ScheduleHandle> {
    if !disable {
        let interval = interval.clamp(1, 60) as u64;
        info!(
            "Starting service routine thread scheduler with interval of {} Minutes",
            interval
        );
        let mut service_scheduler = Scheduler::new();
        service_scheduler
            .every(Interval::Minutes(interval as u32))
            .run(service::service_routine);
        Some(service_scheduler.watch_thread(std::time::Duration::new(interval, 0)))
    } else {
        warn!("Service routine is disabled! This is not recommended and might lead to system instability!");
        None
    }
}
