// extern crate clearscreen;
// #![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
extern crate opencv;
use opencv::core::Size;
use opencv::prelude::*;
use opencv::videoio::{VideoCapture, VideoWriter, CAP_ANY};
use std::{
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

// Alternative value : //" .'`^\"\\,:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$".;
//const CHAR_MAP: [char; 11] = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@', '"'];
const GRAY_U8: [u8; 11] = [32, 46, 58, 45, 61, 43, 42, 35, 37, 64, 34];
const INDEX: u8 = 24;
const RATIO: i32 = 4;
const WIDTH: i32 = 1280 / RATIO;
const HEIGHT: i32 = 720 / RATIO;
const SIZE: Size = Size::new(WIDTH, HEIGHT);
const MS: Duration = Duration::from_millis(17);
fn main() {
    let original = format!(
        "{}/assets/Howl's Moving Castle - Trailer.mp4",
        std::env::current_dir().unwrap().to_str().unwrap()
    );

    let gray = format!(
        "{}/assets/Howl's Moving Castle - Trailer - Grayscale.mp4",
        std::env::current_dir().unwrap().to_str().unwrap()
    );
    convert_resized_gray(&original, &gray);
    display(&gray);
}

fn convert_resized_gray(original: &str, gray: &str) {
    let Ok(mut vc) = VideoCapture::from_file(original, CAP_ANY) else {
        println!("Error opening video file");
        return;
    };
    match VideoCapture::is_opened(&vc) {
        Ok(opened) => assert!(opened, "Unable to open the file"),
        Err(e) => panic!("Unable to open the file!, {e}"),
    };
    let Ok(mut vr) = VideoWriter::new(
        gray,
        VideoWriter::fourcc('m', 'p', '4', 'v').unwrap(),
        60.0,
        SIZE,
        false,
    ) else {
        println!("Error opening video file");
        return;
    };
    'grayscale_conversion: loop {
        let mut frame = opencv::core::Mat::default();
        match vc.read(&mut frame) {
            Ok(grabbed) => {
                if !grabbed {
                    break 'grayscale_conversion;
                }
            }
            Err(err) => {
                println!("Error opening the frame {err}");
                return;
            }
        };
        let mut gray_frame = opencv::core::Mat::default();
        opencv::imgproc::cvt_color(&frame, &mut gray_frame, opencv::imgproc::COLOR_BGR2GRAY, 0)
            .unwrap();
        let mut resized_frame = opencv::core::Mat::default();
        opencv::imgproc::resize(
            &gray_frame,
            &mut resized_frame,
            SIZE,
            0.0,
            0.0,
            opencv::imgproc::InterpolationFlags::INTER_LINEAR.into(),
        )
        .unwrap();
        vr.write(&resized_frame).unwrap();
    }
    vr.release().unwrap();
    vc.release().unwrap();
}

fn display(gray: &str) {
    let stdout = io::stdout();
    let lock = stdout.lock();
    let mut w = io::BufWriter::with_capacity((((WIDTH + 1) * HEIGHT) * 10) as usize, lock);
    let Ok(mut vc) = VideoCapture::from_file(gray, CAP_ANY) else {
        println!("Error opening video file");
        return;
    };
    match VideoCapture::is_opened(&vc) {
        Ok(opened) => {
            assert!(opened, "Unable to open the file");
        }
        Err(e) => panic!("Unable to open the file!, {e}"),
    };
    let mut frame_char: [u8; (WIDTH + 1) as usize] = [0; (WIDTH + 1) as usize];
    'print: loop {
        let mut frame = opencv::core::Mat::default();
        match vc.read(&mut frame) {
            Ok(grabbed) => {
                if !grabbed {
                    break 'print;
                }
            }
            Err(err) => {
                println!("Error opening the frame {err}");
                return;
            }
        };

        let mut gray_frame = opencv::core::Mat::default();
        opencv::imgproc::cvt_color(&frame, &mut gray_frame, opencv::imgproc::COLOR_BGR2GRAY, 0)
            .unwrap();
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                let val = gray_frame.at_2d::<u8>(row, col).unwrap() / INDEX;
                match usize::try_from(col) {
                    Ok(col) => frame_char[col] = GRAY_U8[val as usize],
                    Err(e) => {
                        eprintln!("Error {e}");
                        return;
                    }
                }
            }
            frame_char[WIDTH as usize] = 10;
            w.write_all(&frame_char).expect("Unable to write to STDOUT");
        }
        w.write_all(b"\x1b[0;0f")
            .expect("Unable to write to STDOUT");

        sleep(MS);
    }
}
