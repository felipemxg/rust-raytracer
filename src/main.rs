#[macro_use]
extern crate bmp;
extern crate libc;

mod camera;
mod hittable;
mod hittable_list;
mod material;
mod random;
mod ray;
mod sphere;
mod vec3;

use bmp::{Image, Pixel};
use camera::*;
use hittable::*;
use hittable_list::*;
use material::*;
use random::*;
use ray::*;
use sphere::*;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use vec3::*;

const ASPECT_RATIO: f32 = 3.0 / 2.0;
const WIDTH: u32 = 1200u32;
const HEIGHT: u32 = (WIDTH as f32 / ASPECT_RATIO) as u32;

struct Profile {
    total_time: Duration,
    total_bounces: AtomicU64,
}

unsafe impl Send for Profile {}
unsafe impl Sync for Profile {}

#[derive(Clone, Copy)]
struct RenderBuffer {
    buffer: *mut Vec3,
    w: u32,
    h: u32,
}

unsafe impl Send for RenderBuffer {}
unsafe impl Sync for RenderBuffer {}

impl RenderBuffer {
    fn new(w: u32, h: u32) -> Self {
        unsafe {
            let buffer =
                libc::malloc(std::mem::size_of::<Vec3>() * w as libc::size_t * h as libc::size_t)
                    as *mut Vec3;
            if buffer.is_null() {
                println!("failed to allocate memory!!!")
            }
            Self { buffer, w, h }
        }
    }

    #[inline(always)]
    fn get_pixel_color(&self, x: u32, y: u32) -> Vec3 {
        unsafe {
            let buffer = self
                .buffer
                .offset(y as isize * self.w as isize)
                .offset(x as isize);
            *buffer
        }
    }
}

#[inline(always)]
fn write_color(
    render_buffer: &mut RenderBuffer,
    x: u32,
    y: u32,
    color: Color,
    samples_per_pixel: u32,
) {
    let px = color * (1.0 / samples_per_pixel as f32);
    let r = px.x.sqrt();
    let g = px.y.sqrt();
    let b = px.z.sqrt();
    unsafe {
        let px = render_buffer
            .buffer
            .offset(y as isize * render_buffer.w as isize)
            .offset(x as isize) as *mut Vec3;
        *px = vec3!(r, g, b);
    }
}

#[inline(always)]
fn ray_color_expand(r: Ray, world: &HittableList, depth: u32) -> Color {
    let unit_direction = r.direction.normalized();
    let t = 0.5 * (unit_direction.y + 1.0);
    let mut result = Vec3::lerp(color!(1.0), t, color!(0.5, 0.7, 1.0));
    let mut scattered = r;
    let mut attenuation = Color::ZERO;

    let mut rec = HitRecord::new();

    for d in 0..depth {
        if d == depth {
            return Color::ZERO;
        }

        if world.hit(scattered, 0.001, f32::MAX, &mut rec) {
            if rec
                .material
                .scatter(scattered, rec, &mut attenuation, &mut scattered)
            {
                result = Vec3::hadamard(attenuation, result);
                continue;
            }

            return Color::ZERO;
        }
    }

    result
}

#[inline(always)]
fn ray_color(r: Ray, world: &HittableList, depth: u32) -> Color {
    let mut rec = HitRecord::new();

    if depth <= 0 {
        return Color::ZERO;
    }

    if world.hit(r, 0.001, f32::MAX, &mut rec) {
        let mut scattered = Ray::new(Vec3::ZERO, Vec3::ZERO);
        let mut attenuation = Color::ZERO;
        if rec
            .material
            .scatter(r, rec, &mut attenuation, &mut scattered)
        {
            let result = ray_color(scattered, world, depth - 1);
            return Vec3::hadamard(attenuation, result);
        }

        return Color::ZERO;
    }

    let unit_direction = r.direction.normalized();
    let t = 0.5 * (unit_direction.y + 1.0);
    Vec3::lerp(color!(1.0), t, color!(0.5, 0.7, 1.0))
    //(1.0 - t) * color!(1.0) + t * color!(0.5, 0.7, 1.0)
}

#[inline(always)]
fn render_tile(
    render_buffer: &mut RenderBuffer,
    world: &HittableList,
    camera: Camera,
    tile_x: u32,
    tile_y: u32,
    tile_x2: u32,
    tile_y2: u32,
    samples_per_pixel: u32,
    max_depth: u32,
) -> u64 {
    let mut bounces = 0u64;
    for y in tile_y..tile_y2 {
        for x in tile_x..tile_x2 {
            let mut color = color!();

            for _ in 0..samples_per_pixel {
                let u = (x as f32 + randomf32()) as f32 / (WIDTH - 1) as f32;
                let v = (y as f32 + randomf32()) as f32 / (HEIGHT - 1) as f32;
                let r = camera.get_ray(u, v);
                color += ray_color(r, world, max_depth);

                bounces += 1;
            }

            write_color(render_buffer, x, HEIGHT - 1 - y, color, samples_per_pixel);
        }
    }

    bounces
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Material::new(MaterialType::Lambertian, color!(0.5, 0.5, 0.5), 0.0);
    world.add_sphere(Sphere::new(vec3!(0, -1000, 0), 1000.0, ground_material));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = randomf32();
            let center = vec3!(
                a as f32 + 0.9 * randomf32(),
                0.2,
                b as f32 + 0.9 * randomf32()
            );

            if (center - vec3!(4, 0.2, 0)).len() > 0.9 {
                let sphere_material = if choose_mat < 0.8 {
                    let albedo = Color::hadamard(Color::random(), Color::random());
                    Material::new(MaterialType::Lambertian, albedo, 0.0)
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = randomf32_range(0.0, 0.5);
                    Material::new(MaterialType::Metal, albedo, fuzz)
                } else {
                    Material::new(MaterialType::Dieletric, Vec3::ZERO, 1.5)
                };

                world.add_sphere(Sphere::new(center, 0.2, sphere_material));
            }
        }
    }

    let material1 = Material::new(MaterialType::Dieletric, Color::ZERO, 1.5);
    world.add_sphere(Sphere::new(vec3!(0, 1, 0), 1.0, material1));

    let material2 = Material::new(MaterialType::Lambertian, color!(0.4, 0.2, 0.1), 1.5);
    world.add_sphere(Sphere::new(vec3!(-4, 1, 0), 1.0, material2));

    let material3 = Material::new(MaterialType::Metal, color!(0.7, 0.6, 0.5), 0.0);
    world.add_sphere(Sphere::new(vec3!(4, 1, 0), 1.0, material3));

    world
}

fn start_raytracer() -> RenderBuffer {
    let samples_per_pixel = 4;
    let max_depth = 4;

    let mut profile = Arc::new(Profile {
        total_time: Duration::ZERO,
        total_bounces: AtomicU64::new(0),
    });

    let total_bounces = Arc::new(AtomicU64::new(0));

    let material_ground = Material::new(MaterialType::Lambertian, color!(0.8, 0.8, 0.0), 0.0);
    let material_center = Material::new(MaterialType::Lambertian, color!(0.1, 0.2, 0.5), 0.0);
    let material_left = Material::new(MaterialType::Dieletric, color!(0, 0, 1), 1.5);
    let material_right = Material::new(MaterialType::Lambertian, color!(0.8, 0.6, 0.2), 0.0);

    let mut world = Arc::new(random_scene());

    let lookfrom = vec3!(13, 2, 3);
    let lookat = vec3!(0, 0, 0);
    let vup = vec3!(0, 1, 0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    let mut image = Image::new(WIDTH, HEIGHT);
    let mut render_buffer = RenderBuffer::new(WIDTH, HEIGHT);

    let before = Instant::now();

    let tile_w = 64;
    let tile_h = tile_w;

    let tile_count_x = (WIDTH + tile_w - 1) / tile_w;
    let tile_count_y = (HEIGHT + tile_h - 1) / tile_h;

    let jobs_count = Arc::new(AtomicU32::new(0));
    let max_jobs = 12u32;

    for y in 0..tile_count_y {
        for x in 0..tile_count_x {
            let tile_x = x * tile_w;
            let tile_y = y * tile_h;
            let tile_x2 = u32::min(tile_x + tile_w, WIDTH);
            let tile_y2 = u32::min(tile_y + tile_h, HEIGHT);

            while jobs_count.load(Ordering::Relaxed) >= max_jobs {}
            jobs_count.fetch_add(1, Ordering::Relaxed);

            //let mut tile = render_buffer.get_tile(tile_x, tile_y, tile_x2, tile_y2);
            let jobs_count_clone = Arc::clone(&jobs_count);
            let world_clone = Arc::clone(&world);
            let profile_clone = Arc::clone(&profile);
            let total_bounces_clone = Arc::clone(&total_bounces);
            thread::spawn(move || {
                let bounces = render_tile(
                    &mut render_buffer,
                    &world_clone,
                    camera,
                    tile_x,
                    tile_y,
                    tile_x2,
                    tile_y2,
                    samples_per_pixel,
                    max_depth,
                );
                total_bounces_clone.fetch_add(bounces, Ordering::Relaxed);
                jobs_count_clone.fetch_sub(1, Ordering::Relaxed);
            });
        }
    }

    // wait for threads to execute
    while jobs_count.load(Ordering::Relaxed) > 0 {}

    for (x, y) in image.coordinates() {
        let px_color = render_buffer.get_pixel_color(x, y);
        image.set_pixel(
            x,
            y,
            px!(
                (256.0 * px_color.x.clamp(0.0, 0.999)) as u32,
                (256.0 * px_color.y.clamp(0.0, 0.999)) as u32,
                (256.0 * px_color.z.clamp(0.0, 0.999)) as u32
            ),
        );
    }

    Arc::get_mut(&mut profile).unwrap().total_time = before.elapsed();

    println!("Total time: {}s", profile.total_time.as_secs_f64());
    println!("Total bounces: {}", total_bounces.load(Ordering::Relaxed));

    let total_time_ns = profile.total_time.as_nanos();
    println!(
        "Performance: {}ns/bounce",
        total_time_ns as f64 / total_bounces.load(Ordering::Relaxed) as f64
    );

    image.save("render.bmp").unwrap();

    render_buffer
}

fn main() {
    start_raytracer();
}
