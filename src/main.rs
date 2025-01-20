use std::fs;
use std::sync::Arc;
use std::path::Path;
use std::fs::File;


use fundsp::hacker32::*;

static SOUNDS_DIR : &str = "sounds";

fn main() {
    println!("MCT audio generator");

    
    let paths = fs::read_dir(SOUNDS_DIR).unwrap();
    for path in paths {
        println!("File name: {}", path.unwrap().path().display())
    }


    let w1 = Wave::load("sounds/вода.mp3").expect("Could not load test.wav");
    let w2 = Wave::load("sounds/колокол.mp3").expect("Could not load test.wav");
    // w1:
    println!("sample rate: {}", w1.sample_rate());
    println!("channels: {}", w1.channels());
    println!("len: {}", w1.len());
    // w2:
    println!("sample rate: {}", w2.sample_rate());
    println!("channels: {}", w2.channels());
    println!("len: {}", w2.len());


    let w1arc = Arc::new(w1);
    let w2arc = Arc::new(w2);

    let mut res = wavech_at(&w1arc, 0, 0, 1457280, None) + wavech_at(&w2arc, 0, 0, 1457280, None);
    //res = res + wavech_at(&w2arc, 0, 0, 1457280, None);
    //print_type_of(&res);

    let w3 = Wave::render(44100.0, 10.0, &mut (res)); 
    let path = std::path::Path::new("test.wav");
    w3.save_wav32(path).expect("Could not save test.wav");

    /*

    assert_eq!(wave1.sample_rate(), wave2.sample_rate());
    assert_eq!(wave1.channels(), wave2.channels());
    assert_eq!(wave1.len(), wave2.len());
    assert_eq!(wave1.at(0, 0), wave2.at(0, 0));
    */

    println!("OK.");
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
