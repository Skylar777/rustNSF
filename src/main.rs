extern crate game-music-emu;


use game_music_emu::{Player, System, Track};

fn main() {
    let system = System::new();
    let mut player = system.create_player();

    // Load the NSF file into a Track object.
    let mut track = Track::from_file("nintendo.nsf").unwrap();

    // Set the track to loop indefinitely.
    track.set_loop(true);

    // Start playing the track.
    player.play(&mut track);

    // Main loop: generate audio samples and send them to the output device.
    loop {
        // Generate a buffer of audio samples.
        let mut buffer = [0.0; 4096];
        let samples = player.play(&mut buffer);

        // If no samples were generated, the track has finished playing.
        if samples == 0 {
            break;
        }

        // Send the samples to the output device or output file.
        // For example, you could use the cpal crate to send the samples to the default audio device:
        // let mut output_stream = cpal::default_output_stream().unwrap();
        // output_stream.write(&buffer[..samples]).unwrap();
    }
}


