extern crate opencv;
use asciifyer::{convert_to_ascii, Dimension};
use opencv::{prelude::*, videoio};
use pbr::ProgressBar;
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut cam = videoio::VideoCapture::from_file("res/video/badapple.mp4", videoio::CAP_ANY)?;
    let mut frame = Mat::default();

    // Extract Frames
    let frame_count = cam.get(videoio::CAP_PROP_FRAME_COUNT)? as usize;
    if std::fs::read_dir("res/frames/")?.count() != frame_count {
        println!("Extracting frames...");
        let mut pb = ProgressBar::new(frame_count as u64);
        pb.format("╢▌▌░╟");
        for i in 0..frame_count {
            pb.inc();
            cam.read(&mut frame)?;
            opencv::imgcodecs::imwrite(
                &(format!("res/frames/{}.jpg", i)),
                &frame,
                &opencv::core::Vector::new(),
            )?;
        }
        pb.finish_print("Done!");
    }

    let mut frames: Vec<String> = Vec::new();
    println!("Loading frames...");
    let count = std::fs::read_dir("res/frames/")?.count();
    let mut pb = ProgressBar::new(count as u64);
    pb.format("╢▌▌░╟");
    for i in 0..count {
        pb.inc();
        let ascii = convert_to_ascii(
            &format!("res/frames/{}.jpg", i),
            Some(Dimension::new(80, 80)),
        );
        frames.push(ascii);
    }
    pb.finish_print("Done!");

    let (_stream, stream_handle) = OutputStream::try_default()?;
    let file = BufReader::new(File::open("res/audio/badapple.mp3")?);
    let source = Decoder::new(file)?;
    stream_handle.play_raw(source.convert_samples())?;

    let mut fps = fps_clock::FpsClock::new(33);
    for i in frames {
        print!("{}\x1B[2J\x1B[1;1H", i);
        fps.tick();
    }
    Ok(())
}
