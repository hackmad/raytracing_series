# Raytracing Series in Rust

Reference: https://raytracing.github.io

## Credits

- [NASA Visible Earth](https://visibleearth.nasa.gov/images/73909/december-blue-marble-next-generation-w-topography-and-bathymetry) for the image texture.

## Releases

Completed work from each book is tagged as a release which can be found
[here](https://github.com/hackmad/raytracing_series/releases).

The images shown below are based on those versions. Not all scenes are
available in each release and may look different due to changes in the
algorithms in successive books.

## Building and Running

Build debug profile. The executable will be `target/debug/raytracing_series`.

```bash
cargo build
```

Use `--release` when building/running for faster executable. The executable
will be `target/release/raytracing_series`.

```bash
cargo build --release
```

Run with program arguments (note that `--` is needed after arguments for cargo
and before program arguments can be supplied):

```bash
cargo run -- --scene cornell_box -w 1200 -h 600 -o image.png
```

```bash
./target/debug/raytracing_series --scene cornell_box -w 1200 -h 600 -o image.png
```

Get help on program arguments:

```bash
cargo run -- --help
```

```bash
./target/debug/raytracing_series --help
```

### Raytracing in One Weekend

| Title              | Image                                                     | Title           | Image                                               |
| ------------------ | --------------------------------------------------------- | --------------- | --------------------------------------------------- |
| Lambertian Diffuse | <img src="./images/lambertian_diffuse.png" width="200" /> | Metal           | <img src="./images/metal.png" width="200" />        |
| Dielectric         | <img src="./images/dielectric.png" width="200" />         | Wide Angle Lens | <img src="./images/wide_angle.png" width="200" />   |
| Telephoto Lens     | <img src="./images/telephoto.png" width="200" />          | Defocus Blur    | <img src="./images/defocus_blur.png" width="200" /> |
| Final Image        | <img src="./images/final_one_weekend.png" width="200" />  |                 |                                                     |

### Raytracing The Next Week

| Title                   | Image                                                        | Title                       | Image                                                  |
| ----------------------- | ------------------------------------------------------------ | --------------------------- | ------------------------------------------------------ |
| Motion Blur             | <img src="./images/motion_blur.png" width="200" />           | Motion Blur w/ BVH          | <img src="./images/motion_blur_bvh.png" width="200" /> |
| Final Image w/ BVH      | <img src="./images/final_one_weekend_bvh.png" width="200" /> | Solid &amp; Checker Texture | <img src="./images/checkered_floor.png" width="200" /> |
| Checkered Spheres       | <img src="./images/checkered_spheres.png" width="200" />     | Perlin Noise Texture        | <img src="./images/perlin_spheres.png" width="200" />  |
| Earth Texture           | <img src="./images/earth.png" width="200" />                 | XY Rectangle &amp; Lights   | <img src="./images/simple_light.png" width="200" />    |
| Empty Cornell Box       | <img src="./images/empty_cornell_box.png" width="200" />     | Standard Cornell Box        | <img src="./images/cornell_box.png" width="200" />     |
| Smoke and Fog           | <img src="./images/smoke_and_fog.png" width="200" />         | Final Image                 | <img src="./images/final_next_week.png" width="200" /> |
| Rotation around X, Y, Z | <img src="./images/rotate_spheres.png" width="200" />        |                             |                                                        |

### Raytracing The Rest Of Your Life

| Title                | Image                                                          | Title                | Image                                                       |
| -------------------- | -------------------------------------------------------------- | -------------------- | ----------------------------------------------------------- |
| Standard Cornell Box | <img src="./images/cornell_box_2.png" width="200" />           | Specular Reflections | <img src="./images/specular_reflections.png" width="200" /> |
| Final Image          | <img src="./images/final_rest_of_your_life.png" width="200" /> |                      |                                                             |

#### Timing Benchmarks

##### Single-threaded

Without Bounding Volume Hierarchy:

```bash
target/release/raytracing_series --scene final_one_weekend -w 1200 -h 600 \
    --seed 8589869056 -t 1 -o final_one_weekend.png

HittableList: 0.000061101 seconds
Done: 28.45 minutes
```

With Bounding Volume Hierarchy:

```bash
target/release/raytracing_series --scene final_one_weekend  --bvh -w 1200 -h 600 \
    --seed 8589869056 -t 1 -o final_one_weekend_bvh.png

BVH: 0.000398774 seconds
Done: 9.33 minutese
```

##### Multi-threaded

Without Bounding Volume Hierarchy:

````bash
target/release/raytracing_series --scene final_one_weekend -w 1200 -h 600 \
    --seed 8589869056 -t 4 -o final_one_weekend.png

HittableList: 0.000035394 seconds
Done: 10.89 minutes
```

With Bounding Volume Hierarchy:

```bash
target/release/raytracing_series --scene final_one_weekend --bvh -w 1200 -h 600 \
    --seed 8589869056 -t 4 -o final_one_weekend_bvh.png

BVH: 0.000471452 seconds
Done: 5.14 minutese
````
