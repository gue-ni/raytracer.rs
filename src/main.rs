use raytracer::*;
use std::env;
use std::path::Path;
use std::time::Instant;

fn parse(string: &String) -> u32 {
    match string.parse::<u32>() {
        Ok(val) => val,
        Err(_) => panic!("Could not parse {:?}", string),
    }
}

pub fn main() {
    let mut scene: Scene = Scene::new(Vec3f::new(0.68, 0.87, 0.96));

    let z = 5.0;

    // right
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(1.75, 0.5, z), 0.5),
        material: Material::diffuse(Vec3f::fill(0.999)),
    });
    // middle
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(0.0, 0.0, z), 1.0),
        material: Material::diffuse(Vec3f::new(1.0, 0.0, 0.0)),
    });
    // left
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(-1.75, 0.5, z), 0.5),
        material: Material::specular(Vec3f::fill(0.999)),
    });
    // light
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(0.0, -2.0, z), 0.5),
        material: Material::emissive(from_hex(0xffffff), 12.0),
    });

    let r = 100000.0;
    let s = 1.0;
    let w = 4.0;

    let wall = Material::diffuse(Vec3f::fill(0.99));
    let light = Material::emissive(Vec3f::fill(1.0), 0.5);

    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(0.0, -(r + w), 5.0), r),
        material: wall,
    });
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(0.0, r + s, 5.0), r),
        material: wall,
    });
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(-(r + w), 0.0, 5.0), r),
        material: Material::diffuse(Vec3f::new(0.75, 0.25, 0.25)),
    });
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(r + w, 0.0, 5.0), r),
        material: Material::diffuse(Vec3f::new(0.25, 0.75, 0.25)),
    });
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(0.0, 0.0, 5.0 + (r + w)), r),
        material: wall,
    });

    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        panic!("Invalid numer of arguments {:?}", args.len());
    }

    let width = parse(&args[1]);
    let height = parse(&args[2]);
    let samples = parse(&args[3]);
    let bounces = parse(&args[4]);

    println!(
        "{}x{}, samples: {}, bounces: {}",
        width, height, samples, bounces
    );

    let camera = Camera::new(Vec3f::new(0.0, 0.0, 0.0), (width, height));

    let now = Instant::now();
    let image = render(&camera, &scene, samples, bounces);
    let elapsed = now.elapsed();

    println!("Elapsed time: {:.2?}", elapsed);

    let filename1 = format!("render-{}x{}-s{}-b{}.png", width, height, samples, bounces);
    let _filename2 = "render.png".to_string();

    let path = Path::new(&filename1);

    match image.save(&path) {
        Ok(_) => println!("Saved output to {:?}", path),
        Err(_) => panic!("Could not save file {:?}", path),
    };
}

#[test]
fn test_sample() {}
