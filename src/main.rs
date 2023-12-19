extern crate image;
extern crate opencv;
use opencv::prelude::*;
// use std::process::Command;
// use std::fs;

fn main() {
    let char_map = " .:-=+*#%@"; //" .'`^\"\\,:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$".;

    let path = format!(
        "{}/assets/Howl's Moving Castle - Trailer.mp4",
        std::env::current_dir().unwrap().to_str().unwrap()
    );
    let ratio = 3;

    let mut vc = match opencv::videoio::VideoCapture::from_file(&path, opencv::videoio::CAP_ANY) {
        Ok(capture) => capture,
        Err(_) => {
            println!("Error opening video file");
            return;
        }
    };

    let opened = opencv::videoio::VideoCapture::is_opened(&mut vc).unwrap();
    if !opened {
        panic!("Unable to open the file!");
    }
    loop {
        let mut frame = opencv::core::Mat::default();
        vc.read(&mut frame).unwrap();

        let mut gray_frame = opencv::core::Mat::default();
        opencv::imgproc::cvt_color(&frame, &mut gray_frame, opencv::imgproc::COLOR_BGR2GRAY, 0)
            .unwrap();
        let mut resized_frame = opencv::core::Mat::default();
        opencv::imgproc::resize(
            &gray_frame,
            &mut resized_frame,
            (
                frame.size().unwrap().width / ratio,
                frame.size().unwrap().height / ratio,
            )
                .into(),
            0.0,
            0.0,
            opencv::imgproc::InterpolationFlags::INTER_LINEAR.into(),
        )
        .unwrap();
        //let resized_frame = gray_frame.clone();
        for i in 0..resized_frame.rows() {
            for j in 0..resized_frame.cols() {
                let val = resized_frame.at_2d::<u8>(i as i32, j as i32).unwrap() / 30;
                print!("{}", char_map.chars().nth(val as usize).unwrap());
            }
            println!("");
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
        //let _ = Command::new("clear").status().unwrap();
    }
}
