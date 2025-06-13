use crate::config::Config;
use crate::set::{color_scheme, global_theme, wallpaper};
use crate::theme::Style;
use std::sync::{Arc, Barrier};
use std::thread;

// pub fn set(style: &Style) {
//     // TODO: remove need to clone here?
//     let theme = style.desktop_theme.clone();
//     let wallpaper = style.wallpaper.clone();
//     let color_scheme = style.color_scheme.clone();
//
//     let theme_handle = thread::spawn(move || {
//         let _ = global_theme::set(&theme);
//     });
//
//     let wallpaper_handle = thread::spawn(move || {
//         let _ = wallpaper::set(&wallpaper);
//     });
//
//     let color_scheme_handle = thread::spawn(move || {
//         let _ = color_scheme::set(&color_scheme);
//     });
//
//     // Wait for both to finish
//     let _ = theme_handle.join();
//     let _ = wallpaper_handle.join();
//     let _ = color_scheme_handle.join();
// }
pub fn set(style: &Style) {
    let theme = style.desktop_theme.clone();
    let wallpaper = style.wallpaper.clone();
    let color_scheme = style.color_scheme.clone();

    // Barrier for synchronizing thread start
    let barrier = Arc::new(Barrier::new(4)); // 3 worker threads + main

    let barrier1 = Arc::clone(&barrier);
    let theme_handle = thread::spawn(move || {
        barrier1.wait(); // Wait until all threads are ready
        let _ = global_theme::set(&theme);
    });

    let barrier2 = Arc::clone(&barrier);
    let wallpaper_handle = thread::spawn(move || {
        barrier2.wait(); // Wait until all threads are ready
        let _ = wallpaper::set(&wallpaper);
    });

    let barrier3 = Arc::clone(&barrier);
    let color_scheme_handle = thread::spawn(move || {
        barrier3.wait(); // Wait until all threads are ready
        let _ = color_scheme::set(&color_scheme);
    });

    barrier.wait(); // Let the threads go at the same time

    // Wait for all threads to complete
    let _ = theme_handle.join();
    let _ = wallpaper_handle.join();
    let _ = color_scheme_handle.join();
}

pub fn toggle(config: Config) {
    todo!();
}
