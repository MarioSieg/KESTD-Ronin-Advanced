pub mod prelude;
pub mod sys_info;

use crate::config::CoreConfig;
use glfw::*;
use log::info;
use rayon::iter::*;
use std::sync::mpsc::Receiver;

pub struct WindowData {
    pub context: Glfw,
    pub window: Window,
    pub events: Receiver<(f64, WindowEvent)>,
}

impl WindowData {
    pub fn create_window(cfg: &mut CoreConfig) -> WindowData {
        // create window:
        let mut context = init(FAIL_ON_ERRORS).expect("Failed to initialize glfw context!");

        let mut gamma_ramps: Vec<(Vec<u16>, Vec<u16>, Vec<u16>)> = Vec::new();

        context.with_connected_monitors_mut(|_, monitors| {
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

                vids.into_par_iter().for_each(|vid| {
                    info!("{:#?}", vid);
                });
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

        context.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));
        context.window_hint(WindowHint::Resizable(false));
        context.window_hint(WindowHint::Visible(false));

        fn make_windowed(
            glfw: &mut Glfw,
            width: &mut u16,
            height: &mut u16,
        ) -> Option<(Window, Receiver<(f64, WindowEvent)>)> {
            if *width == 0 || *width > 16384 || *width < 800 {
                *width = 1920;
            }
            if *height == 0 || *height > 16384 || *height < 600 {
                *height = 1080;
            }
            if let Some(win) =
                glfw.create_window(*width as _, *height as _, WIN_TITLE, WindowMode::Windowed)
            {
                *width = win.0.get_framebuffer_size().0 as _;
                *height = win.0.get_framebuffer_size().1 as _;
                Some(win)
            } else {
                None
            }
        }

        let (mut window, events) =
            if cfg.display_config.window_mode == crate::config::WindowMode::Windowed {
                make_windowed(
                    &mut context,
                    &mut cfg.display_config.resolution.0,
                    &mut cfg.display_config.resolution.1,
                )
            } else {
                context.with_primary_monitor_mut(|mut ctx, monitor| {
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
        window.set_char_polling(true);
        window.set_mouse_button_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_key_polling(true);
        WindowData {
            context,
            window,
            events,
        }
    }
}
