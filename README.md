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

### Raytracing in One Weekend

![Lambertian Diffuse](./images/lambertian_diffuse.ppm)
![Metal](./images/metal.ppm)
![Dielectric](./images/dielectric.ppm)
![Camera Viewpoint](./images/camera_viewpoint.ppm)
![Camera Field of View](./images/camera_fov.ppm)
![Defocus Blur / Depth of Field](./images/defocus_blur.ppm)
![Final image](./images/random_spheres.ppm)

