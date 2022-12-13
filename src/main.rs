slint::include_modules!();

pub fn main() {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();

    let main_window = MainWindow::new();

    let main_weak = main_window.as_weak();

    let _thread = std::thread::spawn(move || {
        let mut value: f32 = 0.0;

        loop {
            let main_window_copy = main_weak.clone();

            std::thread::sleep(std::time::Duration::from_millis(1000));

            value += 1.0;

            slint::invoke_from_event_loop(move || {
                main_window_copy.unwrap().global::<Logic>().set_value(value)
            })
            .unwrap();
        }
    });

    main_window.run();
}
