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

    if args.len() != 5 {
        panic!("Invalid numer of arguments {:?}", args.len());
    }

    let width = parse(&args[1]);
    let height = parse(&args[2]);
    let samples = parse(&args[3]);
    let bounces = parse(&args[4]);

    /*

    let mut scene: Scene = Scene::new(Vec3f::new(0.68, 0.87, 0.96));

    let z = 8.5;

    // room size
    let aspect_ratio = (width as f32) / (height as f32);
    let h = 3.0;
    let w = h * aspect_ratio;
    let spacing = 2.3;
    let left = -spacing * 2.5;

    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(left + spacing * 1.0, 2.0, z), 1.0),
        material: Material::transparent(Vec3f::from(0.99)),
    });
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(left + spacing * 2.0, 2.0, z), 1.0),
        material: Material::physical(from_hex(0xff9429), 0.2, 0.5),
    });
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(left + spacing * 3.0, 2.0, z), 1.0),
        material: Material::mirror(Vec3f::from(0.95)),
    });
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(left + spacing * 4.0, 2.0, z), 1.0),
        material: Material::physical(Vec3f::new(0.99, 0.01, 0.01), 0.25, 0.85),
    });

    let r = 100000.0;

    // ################## lights ###################
    /*
    let light_radius = 3.0;
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(0.0, -(h + light_radius * 0.85), z), light_radius),
        material: Material::emissive(from_hex(0x99dde7), 12.0),
    });
    */
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(-1.5, -1.75, z - 1.5), 0.5),
        material: Material::emissive(from_hex(0x45b9d3), 17.0),
    });
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(1.5, -1.75, z + 1.5), 0.5),
        material: Material::emissive(from_hex(0xbb349b), 17.0),
    });

    let wall = Material::diffuse(Vec3f::from(0.99));
    let _light = Material::emissive(Vec3f::from(1.0), 1.0);

    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(0.0, -(r + h), z), r),
        material: wall,
    });
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(0.0, r + h, z), r),
        material: wall,
    });
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(-(r + w), 0.0, z), r),
        material: Material::diffuse(Vec3f::new(0.75, 0.25, 0.25)),
    });
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(r + w, 0.0, z), r),
        material: Material::diffuse(Vec3f::new(0.25, 0.75, 0.25)),
    });
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(0.0, 0.0, z + (r + w)), r),
        material: wall,
    });
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(0.0, 0.0, z - (r + w)), r),
        material: wall,
    });
    */

    let json = fs::read_to_string("scenes/scene.json").unwrap();
    let config: ConfigFile = serde_json::from_str(&json).unwrap();

    println!(
        "resolution: ({}, {}) , samples: {}, bounces: {}",
        width, height, samples, bounces
    );

    //let camera = Camera::new(Vec3f::new(0.0, 0.0, 0.0), (width, height));
    let camera = config.camera;
    let scene = config.scene;

    let now = Instant::now();
    let image = Renderer::render(&camera, &scene, samples, bounces);
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
