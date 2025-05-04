use std::process::Command;

use rfd::FileDialog;

fn main() {
    let file = FileDialog::new()
    .add_filter("videos", &["mp4"])
    .set_directory("Videos")
    .pick_file();

    if let Some(input_path) = file {
        let input = input_path.to_str().unwrap();

        let out = FileDialog::new()
        .set_file_name("output.mp3")
        .add_filter("music", &["mp3"])
        .save_file();

        if let Some(output_path) = out {
           let output = output_path.to_str().unwrap(); 

           let _status = Command::new("ffmpeg")
           .args(["-i", input, "-q:a", "0", "-map", "a", output])
           .status()
           .expect("failed");
        }

    }
}