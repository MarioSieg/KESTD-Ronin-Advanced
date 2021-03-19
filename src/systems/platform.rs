use super::System;
use crate::config::{CoreConfig, WindowMode};
use indicatif::HumanBytes;
use log::*;
use rayon::iter::*;
use std::sync::mpsc::Receiver;
use sysinfo::{DiskExt, NetworkExt, ProcessorExt, SystemExt, UserExt};

pub struct PlatformSystem {
    pub glfw: glfw::Glfw,
    pub window: glfw::Window,
    pub events: Receiver<(f64, glfw::WindowEvent)>,
    pub sys_info: sysinfo::System,
}

impl System<()> for PlatformSystem {
    fn initialize(cfg: &mut CoreConfig, _: &()) -> Self {
        let sys_info = Self::get_and_print_system_info();
        let (glfw, window, events) = Self::create_window(cfg);

        PlatformSystem {
            glfw,
            window,
            events,
            sys_info,
        }
    }

    fn prepare(&mut self) {
        self.window.focus();
        self.window.show();
    }

    fn tick(&mut self) -> bool {
        self.glfw.poll_events();
        for (_, _) in glfw::flush_messages(&self.events) {}
        !self.window.should_close()
    }
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
            info!(
                "{}: {}",
                "aes".to_uppercase(),
                std::is_x86_feature_detected!("aes")
            );
            info!(
                "{}: {}",
                "pclmulqdq".to_uppercase(),
                std::is_x86_feature_detected!("pclmulqdq")
            );
            info!(
                "{}: {}",
                "rdrand".to_uppercase(),
                std::is_x86_feature_detected!("rdrand")
            );
            info!(
                "{}: {}",
                "rdseed".to_uppercase(),
                std::is_x86_feature_detected!("rdseed")
            );
            info!(
                "{}: {}",
                "tsc".to_uppercase(),
                std::is_x86_feature_detected!("tsc")
            );
            info!(
                "{}: {}",
                "mmx".to_uppercase(),
                std::is_x86_feature_detected!("mmx")
            );
            info!(
                "{}: {}",
                "sse".to_uppercase(),
                std::is_x86_feature_detected!("sse")
            );
            info!(
                "{}: {}",
                "sse2".to_uppercase(),
                std::is_x86_feature_detected!("sse2")
            );
            info!(
                "{}: {}",
                "sse3".to_uppercase(),
                std::is_x86_feature_detected!("sse3")
            );
            info!(
                "{}: {}",
                "ssse3".to_uppercase(),
                std::is_x86_feature_detected!("ssse3")
            );
            info!(
                "{}: {}",
                "sse4.1".to_uppercase(),
                std::is_x86_feature_detected!("sse4.1")
            );
            info!(
                "{}: {}",
                "sse4.2".to_uppercase(),
                std::is_x86_feature_detected!("sse4.2")
            );
            info!(
                "{}: {}",
                "sse4a".to_uppercase(),
                std::is_x86_feature_detected!("sse4a")
            );
            info!(
                "{}: {}",
                "sha".to_uppercase(),
                std::is_x86_feature_detected!("sha")
            );
            info!(
                "{}: {}",
                "avx".to_uppercase(),
                std::is_x86_feature_detected!("avx")
            );
            info!(
                "{}: {}",
                "avx2".to_uppercase(),
                std::is_x86_feature_detected!("avx2")
            );
            info!(
                "{}: {}",
                "avx512f".to_uppercase(),
                std::is_x86_feature_detected!("avx512f")
            );
            info!(
                "{}: {}",
                "avx512cd".to_uppercase(),
                std::is_x86_feature_detected!("avx512cd")
            );
            info!(
                "{}: {}",
                "avx512er".to_uppercase(),
                std::is_x86_feature_detected!("avx512er")
            );
            info!(
                "{}: {}",
                "avx512pf".to_uppercase(),
                std::is_x86_feature_detected!("avx512pf")
            );
            info!(
                "{}: {}",
                "avx512bw".to_uppercase(),
                std::is_x86_feature_detected!("avx512bw")
            );
            info!(
                "{}: {}",
                "avx512dq".to_uppercase(),
                std::is_x86_feature_detected!("avx512dq")
            );
            info!(
                "{}: {}",
                "avx512vl".to_uppercase(),
                std::is_x86_feature_detected!("avx512vl")
            );
            info!(
                "{}: {}",
                "avx512ifma".to_uppercase(),
                std::is_x86_feature_detected!("avx512ifma")
            );
            info!(
                "{}: {}",
                "avx512vbmi".to_uppercase(),
                std::is_x86_feature_detected!("avx512vbmi")
            );
            info!(
                "{}: {}",
                "avx512vpopcntdq".to_uppercase(),
                std::is_x86_feature_detected!("avx512vpopcntdq")
            );
            info!(
                "{}: {}",
                "avx512vbmi2".to_uppercase(),
                std::is_x86_feature_detected!("avx512vbmi2")
            );
            info!(
                "{}: {}",
                "avx512gfni".to_uppercase(),
                std::is_x86_feature_detected!("avx512gfni")
            );
            info!(
                "{}: {}",
                "avx512vaes".to_uppercase(),
                std::is_x86_feature_detected!("avx512vaes")
            );
            info!(
                "{}: {}",
                "avx512vpclmulqdq".to_uppercase(),
                std::is_x86_feature_detected!("avx512vpclmulqdq")
            );
            info!(
                "{}: {}",
                "avx512vnni".to_uppercase(),
                std::is_x86_feature_detected!("avx512vnni")
            );
            info!(
                "{}: {}",
                "avx512bitalg".to_uppercase(),
                std::is_x86_feature_detected!("avx512bitalg")
            );
            info!(
                "{}: {}",
                "avx512bf16".to_uppercase(),
                std::is_x86_feature_detected!("avx512bf16")
            );
            info!(
                "{}: {}",
                "avx512vp2intersect".to_uppercase(),
                std::is_x86_feature_detected!("avx512vp2intersect")
            );
            info!(
                "{}: {}",
                "f16c".to_uppercase(),
                std::is_x86_feature_detected!("f16c")
            );
            info!(
                "{}: {}",
                "fma".to_uppercase(),
                std::is_x86_feature_detected!("fma")
            );
            info!(
                "{}: {}",
                "bmi1".to_uppercase(),
                std::is_x86_feature_detected!("bmi1")
            );
            info!(
                "{}: {}",
                "bmi2".to_uppercase(),
                std::is_x86_feature_detected!("bmi2")
            );
            info!(
                "{}: {}",
                "abm".to_uppercase(),
                std::is_x86_feature_detected!("abm")
            );
            info!(
                "{}: {}",
                "lzcnt".to_uppercase(),
                std::is_x86_feature_detected!("lzcnt")
            );
            info!(
                "{}: {}",
                "tbm".to_uppercase(),
                std::is_x86_feature_detected!("tbm")
            );
            info!(
                "{}: {}",
                "popcnt".to_uppercase(),
                std::is_x86_feature_detected!("popcnt")
            );
            info!(
                "{}: {}",
                "fxsr".to_uppercase(),
                std::is_x86_feature_detected!("fxsr")
            );
            info!(
                "{}: {}",
                "xsave".to_uppercase(),
                std::is_x86_feature_detected!("xsave")
            );
            info!(
                "{}: {}",
                "xsaveopt".to_uppercase(),
                std::is_x86_feature_detected!("xsaveopt")
            );
            info!("{}: {}", "xsaves", std::is_x86_feature_detected!("xsaves"));
            info!("{}: {}", "xsavec", std::is_x86_feature_detected!("xsavec"));
            info!(
                "{}: {}",
                "cmpxchg16b".to_uppercase(),
                std::is_x86_feature_detected!("cmpxchg16b")
            );
            info!(
                "{}: {}",
                "adx".to_uppercase(),
                std::is_x86_feature_detected!("adx")
            );
            info!(
                "{}: {}",
                "rtm".to_uppercase(),
                std::is_x86_feature_detected!("rtm")
            );
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

        let mut gamma_ramps: Vec<(Vec<u16>, Vec<u16>, Vec<u16>)> = Vec::new();

        glfw.with_connected_monitors_mut(|_, monitors| {
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

                let gamma_ramp = monitor.get_gamma_ramp();
                let gamma_red = gamma_ramp.red;
                let gamma_green = gamma_ramp.green;
                let gamma_blue = gamma_ramp.blue;

                info!("Gamma ramp red entries: {}", gamma_red.len());
                info!("Gamma ramp green entries: {}", gamma_green.len());
                info!("Gamma ramp blue entries: {}", gamma_blue.len());

                gamma_ramps.push((gamma_red, gamma_green, gamma_blue));

                let vids = monitor.get_video_modes();
                info!("Video modes: {}", vids.len());

                for (j, vid) in vids.iter().enumerate() {
                    info!("Video mode: {}", j + 1);
                    info!("Width: {}", vid.width);
                    info!("Height: {}", vid.height);
                    info!("Refresh rate: {}Hz", vid.refresh_rate);
                    info!("R-Bits: {}", vid.red_bits);
                    info!("G-Bits: {}", vid.green_bits);
                    info!("B-Bits: {}", vid.blue_bits);
                }
            }
        });

        info!("Calculating gamma ramps...");

        for (i, ramp) in gamma_ramps.iter().enumerate() {
            let (r, g, b) = ramp;
            let r_len = r.len() as u128;
            let g_len = g.len() as u128;
            let b_len = b.len() as u128;
            let r_sum: u128 = r
                .into_par_iter()
                .fold_with(0_u128, |a: u128, b: &u16| a.wrapping_add(*b as u128))
                .reduce(|| 0, u128::wrapping_add);

            let g_sum: u128 = g
                .into_par_iter()
                .fold_with(0_u128, |a: u128, b: &u16| a.wrapping_add(*b as u128))
                .reduce(|| 0, u128::wrapping_add);

            let b_sum: u128 = b
                .into_par_iter()
                .fold_with(0_u128, |a: u128, b: &u16| a.wrapping_add(*b as u128))
                .reduce(|| 0, u128::wrapping_add);
            info!("Gamma ramp: {}, Red average: {}", i + 1, r_sum / r_len);
            info!("Gamma ramp: {}, Green average: {}", i + 1, g_sum / g_len);
            info!("Gamma ramp: {}, Blue average: {}", i + 1, b_sum / b_len);
        }

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
            glfw.window_hint(glfw::WindowHint::Visible(false));
            if let Some(win) = glfw.create_window(
                *width as _,
                *height as _,
                WIN_TITLE,
                glfw::WindowMode::Windowed,
            ) {
                *width = win.0.get_framebuffer_size().0 as _;
                *height = win.0.get_framebuffer_size().1 as _;
                Some(win)
            } else {
                None
            }
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
