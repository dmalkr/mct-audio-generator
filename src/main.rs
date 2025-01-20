use std::fs;
use std::path::Path;
use std::fs::File;

use symphonia::core::audio::{ SampleBuffer, AudioBufferRef };
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;


static SOUNDS_DIR : &str = "sounds";

mod output;

fn main() {
    println!("Hello, world!");


    let paths = fs::read_dir(SOUNDS_DIR).unwrap();
    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }




    // Create a media source. Note that the MediaSource trait is automatically implemented for File,
    // among other types.
    let file = Box::new(File::open(Path::new("sounds/x.mp3")).unwrap());

    // Create the media source stream using the boxed media source from above.
    let mss = MediaSourceStream::new(file, Default::default());

    // Create a hint to help the format registry guess what format reader is appropriate. In this
    // example we'll leave it empty.
    let mut hint = Hint::new();
    hint.with_extension("mp3");

    // Use the default options when reading and decoding.
    let format_opts: FormatOptions = Default::default();
    let metadata_opts: MetadataOptions = Default::default();
    let decoder_opts: DecoderOptions = Default::default();

    // Probe the media source stream for a format.
    let probed =
        symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts).unwrap();

    // Get the format reader yielded by the probe operation.
    let mut format = probed.format;

    // Get the default track.
    let track = format.default_track().unwrap();

    // Create a decoder for the track.
    let mut decoder =
        symphonia::default::get_codecs().make(&track.codec_params, &decoder_opts).unwrap();

    // Store the track identifier, we'll use it to filter packets.
    let track_id = track.id;

    println!("Track id: {}", track_id);

    let mut audio_output = None; 

    // The decode loop.
    loop {
        // Get the next packet from the media format.
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(Error::ResetRequired) => {
                // The track list has been changed. Re-examine it and create a new set of decoders,
                // then restart the decode loop. This is an advanced feature and it is not
                // unreasonable to consider this "the end." As of v0.5.0, the only usage of this is
                // for chained OGG physical streams.
                unimplemented!();
            }
            Err(err) => {
                // A unrecoverable error occurred, halt decoding.
                panic!("{}", err);
            }
        };

        // Consume any new metadata that has been read since the last packet.
        while !format.metadata().is_latest() {
            // Pop the old head of the metadata queue.
            format.metadata().pop();

            // Consume the new metadata at the head of the metadata queue.
        }

        // If the packet does not belong to the selected track, skip over it.
        if packet.track_id() != track_id {
            continue;
        }

        // Decode the packet into audio samples.
        match decoder.decode(&packet) {
            Ok(decoded) => {
                // Consume the decoded audio samples (see below).
                println!("DECODED");
                //println!("Decoded: {}", decoded);
                println!("Packet: {} {} {}", &packet.track_id(), &packet.trim_start(), &packet.dur());
                //play(probed.format, track, seek, &decode_opts, no_progress)


                // If the audio output is not open, try to open it.
                if audio_output.is_none() {
                    // Get the audio buffer specification. This is a description of the decoded
                    // audio buffer's sample format and sample rate.
                    let spec = *decoded.spec();

                    // Get the capacity of the decoded buffer. Note that this is capacity, not
                    // length! The capacity of the decoded buffer is constant for the life of the
                    // decoder, but the length is not.
                    let duration = decoded.capacity() as u64;

                    // Try to open the audio output.
                    audio_output.replace(output::try_open(spec, duration).unwrap());
                }
                else {
                    // TODO: Check the audio spec. and duration hasn't changed.
                }

                // Write the decoded audio samples to the audio output if the presentation timestamp
                // for the packet is >= the seeked position (0 if not seeking).
                if let Some(ref mut audio_output) = audio_output {
                    //println!("{}", std::any::type_name::<T>());
                    print_type_of(&decoded);
                    println!("rate: {}", decoded.spec().rate);
                    println!("channels: {}", decoded.spec().channels.count());
                    match decoded {
                        AudioBufferRef::F32(ref buf) => { 
                            println!("AudioBufferRef::F32(_buf)");
                            print_type_of(&buf);
                        }
                        //AudioBufferRef::U8(ref _buf)  => { println!("AudioBufferRef::U8"); }
                        //AudioBufferRef::U16(ref _buf) => { println!("AudioBufferRef::U16(_buf)"); }
                        //AudioBufferRef::U24(ref _buf) => { println!("AudioBufferRef::U24(_buf)"); }
                        //AudioBufferRef::U32(ref _buf) => { println!("AudioBufferRef::U32(_buf)"); }
                        //AudioBufferRef::S8(ref _buf)  => { println!("AudioBufferRef::S8(_buf) "); }
                        //AudioBufferRef::S16(ref _buf) => { println!("AudioBufferRef::S16(_buf)"); }
                        //AudioBufferRef::S24(ref _buf) => { println!("AudioBufferRef::S24(_buf)"); }
                        //AudioBufferRef::S32(ref _buf) => { println!("AudioBufferRef::S32(_buf)"); }
                        //AudioBufferRef::F64(ref _buf) => { println!("AudioBufferRef::F64(_buf)"); }
                        _ => {
                            unimplemented!();
                        }
                    }
                    //let &p = decoded.planes();
                    //let &pp = p.planes();
                    //println!("decoded len: {}", pp.size());
                    audio_output.write(decoded).unwrap()
                }



            }
            Err(Error::IoError(_)) => {
                // The packet failed to decode due to an IO error, skip the packet.
                continue;
            }
            Err(Error::DecodeError(_)) => {
                // The packet failed to decode due to invalid data, skip the packet.
                continue;
            }
            Err(err) => {
                // An unrecoverable error occurred, halt decoding.
                panic!("{}", err);
            }
        }
    }

}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
