use raytracer::*;
use std::path::Path;
use std::time::Instant;

pub fn main() {
    let mut scene: Scene = Scene::new(Vec3f::new(0.68, 0.87, 0.96));

    // right
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(1.75, 0.5, 4.0), 0.5),
        material: Material::diffuse(Vec3f::new(0.0, 1.0, 0.0)),
    });
    // middle
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(0.0, 0.0, 4.0), 1.0),
        material: Material::diffuse(Vec3f::new(1.0, 0.0, 0.0)),
    });
    // left
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(-1.75, 0.5, 4.0), 0.5),
        material: Material::specular(Vec3f::new(1.0, 1.0, 1.0)),
    });

    // light
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(0.0, -1.5, 4.0), 0.2),
        material: Material::emissive(from_hex(0xffffff), 0.25),
    });

    let r = 100000.0;
    let s = 1.0;
    let w = 4.0;

    let wall = Material::diffuse(Vec3f::fill(1.0));
    let light = Material::emissive(Vec3f::fill(1.0), 0.5);

    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(0.0, -(r + w), 5.0), r),
        material: light,
    });
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(0.0, r + s, 5.0), r),
        material: wall,
    });
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(-(r + w), 0.0, 5.0), r),
        material: wall,
    });
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(r + w, 0.0, 5.0), r),
        material: wall,
    });
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(0.0, 0.0, 5.0 + (r + w)), r),
        material: wall,
    });

    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 480;
    const SAMPLES: u32 = 512;
    const BOUNCES: u32 = 5;

    let camera = Camera::new(Vec3f::new(0.0, 0.0, 0.0), (WIDTH, HEIGHT));

    let now = Instant::now();
    let image = render(&camera, &scene, SAMPLES, BOUNCES);
    let elapsed = now.elapsed();

    println!(
        "{}x{}, samples: {}, bounces: {}",
        WIDTH, HEIGHT, SAMPLES, BOUNCES
    );
    println!("Elapsed time: {:.2?}", elapsed);

    let filename1 = format!(
        "render-{}x{}-s{}-b{}.png",
        WIDTH, HEIGHT, SAMPLES, BOUNCES
    );
    let filename2 = format!("render.png");

    let path = Path::new(&filename1);

    match image.save(&path) {
        Ok(_) => println!("Saved output to {:?}", path),
        Err(_) => panic!("Could not save file {:?}", path),
    };
}

#[test]
fn test_sample() {}
