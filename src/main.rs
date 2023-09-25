use image::RgbImage;
use raytracer::*;
use std::env;
use std::fs;
use std::path::Path;
use std::time::Instant;

use std::time::SystemTime;

fn get_sys_time_in_secs() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

fn parse(string: &String) -> u32 {
    match string.parse::<u32>() {
        Ok(val) => val,
        Err(_) => panic!("Could not parse {:?}!", string),
    }
}

fn save_image(image: &RgbImage, filename: &String) {
    let path = Path::new(filename);
    match image.save(&path) {
        Ok(_) => println!("Saved image to {:?}!", path),
        Err(_) => panic!("Could not save image {:?}!", path),
    };
}

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 6 {
        println!(
            "Usage: {} <width> <height> <samples> <bounces> <scene json>",
            args[0]
        );
        panic!("Invalid numer of arguments {:?}", args.len());
    }

    let width = parse(&args[1]);
    let height = parse(&args[2]);
    let samples = parse(&args[3]);
    let bounces = parse(&args[4]);
    let scene_path = &args[5];

    let json = fs::read_to_string(scene_path).unwrap();
    let config: ConfigFile = serde_json::from_str(&json).unwrap();

    let camera = Camera::new(config.camera.position, (width, height));

    println!("Config: {}", scene_path);
    println!(
        "resolution: ({}, {}) , samples: {}, bounces: {}",
        width, height, samples, bounces
    );

    let now = Instant::now();
    let image = Renderer::render(&camera, &config.scene, samples, bounces);
    let elapsed = now.elapsed();

    println!("Elapsed time: {:.2?}", elapsed);

    let timestamp = get_sys_time_in_secs();
    let filename = format!(
        "img/render/render-{}-{}x{}-s{}-b{}.png",
        timestamp, width, height, samples, bounces
    );
    save_image(&image, &String::from("render.png"));
    save_image(&image, &String::from("img/latest.png"));
    save_image(&image, &filename);
}
