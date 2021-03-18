use super::{CoreConfig, System};
use glfw::Context;
use log::*;
use std::sync::mpsc::Receiver;
use sysinfo::{NetworkExt, ProcessorExt, SystemExt, UserExt};

pub struct PlatformSystem {
    pub glfw: glfw::Glfw,
    pub window: glfw::Window,
    pub events: Receiver<(f64, glfw::WindowEvent)>,
    pub sys_info: sysinfo::System,
}

impl System for PlatformSystem {
    fn initialize(_cfg: &CoreConfig) -> Self {
        // print system info:
        let mut sys_info = sysinfo::System::new_all();
        sys_info.refresh_all();

        info!("CPU: {}", sys_info.get_global_processor_info().get_brand());
        info!("Logical cores: {}", num_cpus::get());

        for component in sys_info.get_components() {
            info!("{:?}", component);
        }

        for disk in sys_info.get_disks() {
            info!("{:?}", disk);
        }

        info!(
            "Total memory: {} GB",
            sys_info.get_total_memory() as f32 / 1024.0 / 1024.0
        );
        info!(
            "Used memory : {} GB",
            sys_info.get_used_memory() as f32 / 1024.0 / 1024.0
        );
        info!(
            "Total swap  : {} GB",
            sys_info.get_total_swap() as f32 / 1024.0 / 1024.0
        );
        info!(
            "Used swap   : {} GB",
            sys_info.get_used_swap() as f32 / 1024.0 / 1024.0
        );

        info!("System name:             {:?}", sys_info.get_name());
        info!(
            "System kernel version:   {:?}",
            sys_info.get_kernel_version()
        );
        info!("System OS version:       {:?}", sys_info.get_os_version());
        info!("System host name:        {:?}", sys_info.get_host_name());

        for user in sys_info.get_users() {
            info!(
                "User {} is in {} groups",
                user.get_name(),
                user.get_groups().len()
            );
        }

        for (interface_name, data) in sys_info.get_networks() {
            info!(
                "[{}] in: {}, out: {}",
                interface_name,
                data.get_received(),
                data.get_transmitted(),
            );
        }

        // create window:
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let (mut window, events) = glfw
            .create_window(
                800,
                600,
                "KESTD Ronin Advanced - Simulation",
                glfw::WindowMode::Windowed,
            )
            .unwrap();
        window.make_current();

        PlatformSystem {
            glfw,
            window,
            events,
            sys_info,
        }
    }

    fn tick(&mut self) -> bool {
        self.window.swap_buffers();
        self.glfw.poll_events();
        for (_, _) in glfw::flush_messages(&self.events) {}
        self.window.should_close()
    }
}

impl Drop for PlatformSystem {
    fn drop(&mut self) {
        self.window.hide()
    }
}
