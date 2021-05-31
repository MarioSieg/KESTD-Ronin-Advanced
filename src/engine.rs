use super::config::CoreConfig;
use super::resources::ResourceManager;
use super::scenery::Scenery;
use super::scheduler::{self, ScheduleHandle};
use super::systems::SystemSupervisor;
use humantime::Duration;
use log::info;
use std::process;
use std::time::Instant;

pub struct Engine {
    pub config: CoreConfig,
    pub systems: SystemSupervisor,
    pub scenery: Box<Scenery>,
    pub resource_manager: ResourceManager,
    pub service_scheduler_thread: Option<ScheduleHandle>,
}

impl Engine {
    pub fn initialize() -> Box<Self> {
        super::panic_hook::install();
        let clock = Instant::now();
        let _ = super::logger::create();

        println!("{}", LOGO);
        info!("Initializing KESTD Ronin simulation system...");
        info!("PID: {}", process::id());
        info!(
            "Working directory: {:?}",
            std::env::current_dir().unwrap_or_default()
        );
        info!(
            "Executable: {:?}",
            std::env::current_exe().unwrap_or_default()
        );

        let mut config = CoreConfig::load();
        let systems = SystemSupervisor::initialize(&mut config);
        let mut resource_manager = ResourceManager::with_capacity(
            config.application_config.default_resource_cache_capacity,
        );

        let disable_service_routine = config.application_config.disable_service_routine;
        let service_routine_interval = config.application_config.service_routine_minute_interval;
        let service_scheduler_thread =
            scheduler::launch_fixed_routine(disable_service_routine, service_routine_interval);

        info!("Initializing scenery...");
        let scenery_clock = Instant::now();
        let scenery = Scenery::default_preset(&systems, &mut resource_manager);
        info!(
            "Scenery is initialized! Time: {}",
            Duration::from(scenery_clock.elapsed())
        );

        let this = Self {
            config,
            systems,
            scenery,
            resource_manager,
            service_scheduler_thread,
        };

        info!(
            "System online! Boot time: {}",
            Duration::from(clock.elapsed())
        );
        Box::new(this)
    }

    pub fn run(&mut self) -> u32 {
        use std::io::Write;

        info!("Preparing systems...");
        self.systems.prepare_all(&mut self.scenery);

        info!("Executing simulation...");
        let _ = std::io::stdout().flush();
        let clock = Instant::now();

        let mut cycles = 0;
        while self.tick() {
            cycles += 1;
        }

        info!(
            "Simulation stopped. Simulated for {} with {} cycles!",
            Duration::from(clock.elapsed()),
            cycles
        );
        cycles
    }

    pub fn shutdown(&mut self) {
        println!("Shutting down simulation system...");
    }

    fn tick(&mut self) -> bool {
        self.systems.tick_all(&mut self.scenery)
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: mimalloc::MiMalloc = mimalloc::MiMalloc;

const LOGO: &str = r#"
KKKKKKKKK    KKKKKKKEEEEEEEEEEEEEEEEEEEEEE   SSSSSSSSSSSSSSS TTTTTTTTTTTTTTTTTTTTTTTDDDDDDDDDDDDD
K:::::::K    K:::::KE::::::::::::::::::::E SS:::::::::::::::ST:::::::::::::::::::::TD::::::::::::DDD
K:::::::K    K:::::KE::::::::::::::::::::ES:::::SSSSSS::::::ST:::::::::::::::::::::TD:::::::::::::::DD
K:::::::K   K::::::KEE::::::EEEEEEEEE::::ES:::::S     SSSSSSST:::::TT:::::::TT:::::TDDD:::::DDDDD:::::D
KK::::::K  K:::::KKK  E:::::E       EEEEEES:::::S            TTTTTT  T:::::T  TTTTTT  D:::::D    D:::::D
  K:::::K K:::::K     E:::::E             S:::::S                    T:::::T          D:::::D     D:::::D
  K::::::K:::::K      E::::::EEEEEEEEEE    S::::SSSS                 T:::::T          D:::::D     D:::::D
  K:::::::::::K       E:::::::::::::::E     SS::::::SSSSS            T:::::T          D:::::D     D:::::D
  K:::::::::::K       E:::::::::::::::E       SSS::::::::SS          T:::::T          D:::::D     D:::::D
  K::::::K:::::K      E::::::EEEEEEEEEE          SSSSSS::::S         T:::::T          D:::::D     D:::::D
  K:::::K K:::::K     E:::::E                         S:::::S        T:::::T          D:::::D     D:::::D
KK::::::K  K:::::KKK  E:::::E       EEEEEE            S:::::S        T:::::T          D:::::D    D:::::D
K:::::::K   K::::::KEE::::::EEEEEEEE:::::ESSSSSSS     S:::::S      TT:::::::TT      DDD:::::DDDDD:::::D
K:::::::K    K:::::KE::::::::::::::::::::ES::::::SSSSSS:::::S      T:::::::::T      D:::::::::::::::DD
K:::::::K    K:::::KE::::::::::::::::::::ES:::::::::::::::SS       T:::::::::T      D::::::::::::DDD
KKKKKKKKK    KKKKKKKEEEEEEEEEEEEEEEEEEEEEE SSSSSSSSSSSSSSS         TTTTTTTTTTT      DDDDDDDDDDDDD
"#;
