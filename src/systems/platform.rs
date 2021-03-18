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

impl PlatformSystem {
    fn get_and_print_system_info() -> sysinfo::System {
        let mut sys_info = sysinfo::System::new_all();
        sys_info.refresh_all();

        info!("CPU: {}", sys_info.get_global_processor_info().get_brand());
        info!("CPU cores (logical): {}", num_cpus::get());
        info!("CPU cores (physical): {}", num_cpus::get_physical());

        #[cfg(target_arch = "x86_64")]
        {
            info!("{}: {}", "aes", std::is_x86_feature_detected!("aes"));
            info!(
                "{}: {}",
                "pclmulqdq",
                std::is_x86_feature_detected!("pclmulqdq")
            );
            info!("{}: {}", "rdrand", std::is_x86_feature_detected!("rdrand"));
            info!("{}: {}", "rdseed", std::is_x86_feature_detected!("rdseed"));
            info!("{}: {}", "tsc", std::is_x86_feature_detected!("tsc"));
            info!("{}: {}", "mmx", std::is_x86_feature_detected!("mmx"));
            info!("{}: {}", "sse", std::is_x86_feature_detected!("sse"));
            info!("{}: {}", "sse2", std::is_x86_feature_detected!("sse2"));
            info!("{}: {}", "sse3", std::is_x86_feature_detected!("sse3"));
            info!("{}: {}", "ssse3", std::is_x86_feature_detected!("ssse3"));
            info!("{}: {}", "sse4.1", std::is_x86_feature_detected!("sse4.1"));
            info!("{}: {}", "sse4.2", std::is_x86_feature_detected!("sse4.2"));
            info!("{}: {}", "sse4a", std::is_x86_feature_detected!("sse4a"));
            info!("{}: {}", "sha", std::is_x86_feature_detected!("sha"));
            info!("{}: {}", "avx", std::is_x86_feature_detected!("avx"));
            info!("{}: {}", "avx2", std::is_x86_feature_detected!("avx2"));
            info!(
                "{}: {}",
                "avx512f",
                std::is_x86_feature_detected!("avx512f")
            );
            info!(
                "{}: {}",
                "avx512cd",
                std::is_x86_feature_detected!("avx512cd")
            );
            info!(
                "{}: {}",
                "avx512er",
                std::is_x86_feature_detected!("avx512er")
            );
            info!(
                "{}: {}",
                "avx512pf",
                std::is_x86_feature_detected!("avx512pf")
            );
            info!(
                "{}: {}",
                "avx512bw",
                std::is_x86_feature_detected!("avx512bw")
            );
            info!(
                "{}: {}",
                "avx512dq",
                std::is_x86_feature_detected!("avx512dq")
            );
            info!(
                "{}: {}",
                "avx512vl",
                std::is_x86_feature_detected!("avx512vl")
            );
            info!(
                "{}: {}",
                "avx512ifma",
                std::is_x86_feature_detected!("avx512ifma")
            );
            info!(
                "{}: {}",
                "avx512vbmi",
                std::is_x86_feature_detected!("avx512vbmi")
            );
            info!(
                "{}: {}",
                "avx512vpopcntdq",
                std::is_x86_feature_detected!("avx512vpopcntdq")
            );
            info!(
                "{}: {}",
                "avx512vbmi2",
                std::is_x86_feature_detected!("avx512vbmi2")
            );
            info!(
                "{}: {}",
                "avx512gfni",
                std::is_x86_feature_detected!("avx512gfni")
            );
            info!(
                "{}: {}",
                "avx512vaes",
                std::is_x86_feature_detected!("avx512vaes")
            );
            info!(
                "{}: {}",
                "avx512vpclmulqdq",
                std::is_x86_feature_detected!("avx512vpclmulqdq")
            );
            info!(
                "{}: {}",
                "avx512vnni",
                std::is_x86_feature_detected!("avx512vnni")
            );
            info!(
                "{}: {}",
                "avx512bitalg",
                std::is_x86_feature_detected!("avx512bitalg")
            );
            info!(
                "{}: {}",
                "avx512bf16",
                std::is_x86_feature_detected!("avx512bf16")
            );
            info!(
                "{}: {}",
                "avx512vp2intersect",
                std::is_x86_feature_detected!("avx512vp2intersect")
            );
            info!("{}: {}", "f16c", std::is_x86_feature_detected!("f16c"));
            info!("{}: {}", "fma", std::is_x86_feature_detected!("fma"));
            info!("{}: {}", "bmi1", std::is_x86_feature_detected!("bmi1"));
            info!("{}: {}", "bmi2", std::is_x86_feature_detected!("bmi2"));
            info!("{}: {}", "abm", std::is_x86_feature_detected!("abm"));
            info!("{}: {}", "lzcnt", std::is_x86_feature_detected!("lzcnt"));
            info!("{}: {}", "tbm", std::is_x86_feature_detected!("tbm"));
            info!("{}: {}", "popcnt", std::is_x86_feature_detected!("popcnt"));
            info!("{}: {}", "fxsr", std::is_x86_feature_detected!("fxsr"));
            info!("{}: {}", "xsave", std::is_x86_feature_detected!("xsave"));
            info!(
                "{}: {}",
                "xsaveopt",
                std::is_x86_feature_detected!("xsaveopt")
            );
            info!("{}: {}", "xsaves", std::is_x86_feature_detected!("xsaves"));
            info!("{}: {}", "xsavec", std::is_x86_feature_detected!("xsavec"));
            info!(
                "{}: {}",
                "cmpxchg16b",
                std::is_x86_feature_detected!("cmpxchg16b")
            );
            info!("{}: {}", "adx", std::is_x86_feature_detected!("adx"));
            info!("{}: {}", "rtm", std::is_x86_feature_detected!("rtm"));
        }

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
            HumanBytes(sys_info.get_total_memory())
        );
        info!("Used memory: {} GB", HumanBytes(sys_info.get_used_memory()));
        info!("Total swap: {} GB", HumanBytes(sys_info.get_total_swap()));
        info!("Used swap: {} GB", HumanBytes(sys_info.get_used_swap()));

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

        sys_info
    }

    fn create_window(
        cfg: &mut CoreConfig,
    ) -> (glfw::Glfw, glfw::Window, Receiver<(f64, glfw::WindowEvent)>) {
        // create window:
        let mut glfw =
            glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to initialize glfw context!");

        glfw.with_connected_monitors(|_, monitors| {
            for (i, monitor) in monitors.iter().enumerate() {
                info!("Monitor: {}", i + 1);
                info!(
                    "Name: {}",
                    monitor
                        .get_name()
                        .unwrap_or_else(|| String::from("Unknown"))
                );
                info!("Position: {:?}", monitor.get_pos());
                info!("Phyical size: {:?}", monitor.get_physical_size());
                info!("Content scale: {:?}", monitor.get_content_scale());
                info!("Workarea: {:?}", monitor.get_workarea());

                let vids = monitor.get_video_modes();
                info!("Video modes: {}", vids.len());

                for (j, vid) in vids.iter().enumerate() {
                    info!("Video mode: {}", j + 1);
                    info!("Width: {}", vid.width);
                    info!("Height: {}", vid.height);
                    info!("Refresh rate: {}Hz", vid.refresh_rate);
                    info!("R-Bits: {:b}", vid.red_bits);
                    info!("G-Bits: {:b}", vid.green_bits);
                    info!("B-Bits: {:b}", vid.blue_bits);
                }
            }
        });

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
                *height = 1080;
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

        let (window, events) = if cfg.display_config.window_mode == WindowMode::Windowed {
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
        (glfw, window, events)
    }
}

impl System<()> for PlatformSystem {
    fn initialize(cfg: &mut CoreConfig, _: &()) -> Self {
        let sys_info = Self::get_and_print_system_info();
        let (glfw, mut window, events) = Self::create_window(cfg);

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
