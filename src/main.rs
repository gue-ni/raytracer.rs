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
        Err(_) => panic!("SystemTime before UNIX_EPOCH!"),
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
        eprintln!(
            "Usage: {} <scene_json> <width> <height> <samples> <bounces>",
            args[0]
        );
        std::process::exit(1);
    }

    let width = parse(&args[1]);
    let height = parse(&args[2]);
    let samples = parse(&args[3]);
    let bounces = parse(&args[4]);
    let scene_path = &args[5];

    let json = match fs::read_to_string(scene_path) {
        Ok(value) => value,
        Err(error) => panic!("Failed to read {:?} ({:?})", scene_path, error),
    };

    let config: ConfigFile = match serde_json::from_str(&json) {
        Ok(value) => value,
        Err(error) => panic!("Failed to parse scene: {:?}", error),
    };

    let camera = Camera::new(config.camera.position, (width, height));

    let mut scene = config.scene;

    /*
    for i in 0..scene.objects.len() {
        if 0.0 < scene.objects[i].material.emittance {
            scene.lights.push(i);
        }
    }
    */

    println!("Config: {}", scene_path);
    println!(
        "Resolution: ({}, {}), Samples: {}, Bounces: {}",
        width, height, samples, bounces
    );

    let now = Instant::now();
    let image = Renderer::render(&camera, &scene, samples, bounces);
    let elapsed = now.elapsed();

    println!("Rendered {} pixels in {:.2?}", width * height, elapsed);

    let timestamp = get_sys_time_in_secs();
    let filename = format!(
        "renders/render-{}-{}x{}-s{}-b{}.png",
        timestamp, width, height, samples, bounces
    );
    save_image(&image, &String::from("render.png"));
    save_image(&image, &filename);
}
