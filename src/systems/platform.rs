use super::System;
use crate::config::{CoreConfig, WindowMode};
use indicatif::HumanBytes;
use log::*;
use std::sync::mpsc::Receiver;
use sysinfo::{DiskExt, NetworkExt, ProcessorExt, SystemExt, UserExt};

pub struct PlatformSystem {
    pub glfw: glfw::Glfw,
    pub window: glfw::Window,
    pub events: Receiver<(f64, glfw::WindowEvent)>,
    pub sys_info: sysinfo::System,
}

impl System for PlatformSystem {
    fn initialize(cfg: &mut CoreConfig) -> Self {
        // print system info:
        let mut sys_info = sysinfo::System::new_all();
        sys_info.refresh_all();

        info!("CPU: {}", sys_info.get_global_processor_info().get_brand());
        info!("Logical cores: {}", num_cpus::get());

        for component in sys_info.get_components() {
            info!("{:?}", component);
        }

        for disk in sys_info.get_disks() {
            info!(
                "Disk: {:?}, Type: {:?}, FS: {}, {} / {}",
                disk.get_name(),
                disk.get_type(),
                String::from_utf8_lossy(disk.get_file_system()),
                HumanBytes(disk.get_total_space() - disk.get_available_space()),
                HumanBytes(disk.get_total_space())
            );
        }

        info!(
            "Total memory: {} GB",
            sys_info.get_total_memory() as f32 / 1024.0 / 1024.0
        );
        info!(
            "Used memory: {} GB",
            sys_info.get_used_memory() as f32 / 1024.0 / 1024.0
        );
        info!(
            "Total swap: {} GB",
            sys_info.get_total_swap() as f32 / 1024.0 / 1024.0
        );
        info!(
            "Used swap: {} GB",
            sys_info.get_used_swap() as f32 / 1024.0 / 1024.0
        );

        info!(
            "System name: {}",
            sys_info
                .get_name()
                .unwrap_or_else(|| String::from("Unknown"))
        );
        info!(
            "System kernel version: {}",
            sys_info
                .get_kernel_version()
                .unwrap_or_else(|| String::from("Unknown"))
        );
        info!(
            "System OS version: {}",
            sys_info
                .get_os_version()
                .unwrap_or_else(|| String::from("Unknown"))
        );
        info!(
            "Machine name: {}",
            sys_info
                .get_host_name()
                .unwrap_or_else(|| String::from("Unknown"))
        );

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
        let mut glfw =
            glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to initialize glfw context!");

        const WIN_TITLE: &str = "KESTD Ronin Advanced - Simulation";

        fn make_windowed(
            glfw: &mut glfw::Glfw,
            width: &mut u16,
            height: &mut u16,
        ) -> Option<(glfw::Window, Receiver<(f64, glfw::WindowEvent)>)> {
            if *width == 0 || *width > 16384 || *width < 800 {
                *width = 1920;
            }
            if *height == 0 || *height > 16384 || *height < 600 {
                *height = 1920;
            }
            glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
            glfw.window_hint(glfw::WindowHint::Resizable(false));
            glfw.create_window(
                *width as _,
                *height as _,
                WIN_TITLE,
                glfw::WindowMode::Windowed,
            )
        };

        let (mut window, events) = if cfg.display_config.window_mode == WindowMode::Windowed {
            make_windowed(
                &mut glfw,
                &mut cfg.display_config.resolution.0,
                &mut cfg.display_config.resolution.1,
            )
        } else {
            glfw.with_primary_monitor_mut(|mut ctx, monitor| {
                // if we fail to get the primary monitor, try windowed mode:
                if monitor.is_none() {
                    make_windowed(
                        &mut ctx,
                        &mut cfg.display_config.resolution.0,
                        &mut cfg.display_config.resolution.1,
                    )
                } else {
                    let monitor = monitor.expect("Failed to retrieve primary monitor!");
                    let video_mode = monitor
                        .get_video_mode()
                        .expect("Failed to retrieve primary video mode!");
                    cfg.display_config.resolution.0 = video_mode.width as _;
                    cfg.display_config.resolution.1 = video_mode.height as _;
                    ctx.create_window(
                        cfg.display_config.resolution.0 as _,
                        cfg.display_config.resolution.1 as _,
                        WIN_TITLE,
                        glfw::WindowMode::FullScreen(monitor),
                    )
                }
            })
        }
        .expect("Failed to create window!");

        window.focus();
        window.show();

        PlatformSystem {
            glfw,
            window,
            events,
            sys_info,
        }
    }

    fn tick(&mut self) -> bool {
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
