# Raytracing Series in Rust

Reference: https://raytracing.github.io

## Building and Running

Build debug profile:

```bash
cargo build
```

Build release profile (generally much faster):

```bash
cargo build --release
```

Build and run with default settings:

```bash
cargo run > image.ppm
```

Run compiled versions with default settings:

```bash
./target/debug/raytracing_series > image.ppm
```

```bash
./target/release/raytracing_series > image.ppm
```

Get help on program arguments:

```bash
cargo run -- --help
```

```bash
./target/debug/raytracing_series --help
```

```bash
./target/release/raytracing_series --help
```

## Images

Convert ppm to png using `sips`:

```bash
sips -s format png image.ppm --out image.png
```

### Raytracing in One Weekend

<img src="./images/lambertian_diffuse.png" alt="Lambertian Diffuse" height="100" 
/> <img src="./images/metal.png" alt="Metal" height="100"
/> <img src="./images/dielectric.png" alt="Dielectric" height="100" 
/> <img src="./images/camera_viewpoint.png" alt="Camera Viewpoint" height="100" 
/> <img src="./images/camera_fov.png" alt="Camera Field of View" height="100" 
/> <img src="./images/defocus_blur.png" alt="Defocus Blur / Depth of Field" height="100" 
/> <img src="./images/random_spheres.png" alt="Final image" height="100" />

### Raytracing The Next Week

<img src="./images/motion_blur.png" alt="Motion Blur" height="100" />
