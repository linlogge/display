use std::thread;
use std::time::Duration;

use serde_json::json;

mod http;

slint::include_modules!();

#[derive(Debug)]
struct SongInfo {
    pub title: String,
    pub artist: String,
    pub album: String,
}

pub fn main() {
    let main_window = MainWindow::new();

    let main_weak = main_window.as_weak();

    thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().unwrap();

        runtime.block_on(async {
            loop {
                let main_window_copy = main_weak.clone();

                thread::sleep(Duration::from_millis(1000));

                let req = json!({
                  "Media_Obj": "ActiveInput",
                  "Method": "ActiveInputCmd",
                  "Parameters": {
                    "AudioGetInfo": {"Method": "GetCurrentSongInfo"}
                  }
                });

                let res = http::get(req).await.unwrap();

                let song_dictionary = &res["SongDictionary"];

                let song_info = SongInfo {
                    title: song_dictionary["Title"].as_str().unwrap().to_string(),
                    artist: song_dictionary["Artist"].as_str().unwrap().to_string(),
                    album: song_dictionary["Album"].as_str().unwrap().to_string(),
                };

                println!("{:?}", song_info);

                slint::invoke_from_event_loop(move || {
                    main_window_copy
                        .unwrap()
                        .global::<Logic>()
                        .set_title(song_info.title.into());
                    main_window_copy
                        .unwrap()
                        .global::<Logic>()
                        .set_artist(song_info.artist.into());
                })
                .unwrap();
            }
        });
    });

    main_window.run();
}
