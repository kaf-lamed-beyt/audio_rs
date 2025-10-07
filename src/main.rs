use rodio::Source;
use rodio::source::SineWave;
use std::error::Error;
use std::io::BufReader;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()?;
    let mixer = stream_handle.mixer();

    let mp3 = {
        let file = std::fs::File::open("assets/music.mp3")?;
        let sink = rodio::play(mixer, BufReader::new(file))?;
        sink.set_volume(0.8);
        sink
    };
    println!("playing music.mp3");
    thread::sleep(Duration::from_millis(2000));

    {
        // sine wave typa' shit
        let wave = SineWave::new(740.0)
            .amplify(0.2)
            .take_duration(Duration::from_secs(3));
        mixer.add(wave);
    }
    println!("playing music.ogg");
    thread::sleep(Duration::from_millis(2000));

    let ogg = {
        let file = std::fs::File::open("assets/music.ogg")?;
        let sink = rodio::play(mixer, BufReader::new(file))?;
        sink.set_volume(0.2);
        sink
    };
    println!("playing music.wav");
    thread::sleep(Duration::from_millis(2000));

    let wav = {
        let file = std::fs::File::open("assets/music.wav")?;
        let sink = rodio::play(mixer, BufReader::new(file))?;
        sink.set_volume(0.2);
        sink
    };

    drop(mp3);
    println!("Stopped music.mp3");

    thread::sleep(Duration::from_millis(1500));
    drop(ogg);
    println!("Stopped music.ogg");

    drop(wav);
    println!("Stopped music.wav");

    Ok(())
}
