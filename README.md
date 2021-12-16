imeq-rs aims to quickly compare two images to see if they are the same image.


# Build
Compiles to: `target/release/imeq`
```
cargo build --release
```

# Usage
```
USAGE:
    imeq <IMAGE_1> <IMAGE_2>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <IMAGE_1>    Sets the first image to use
    <IMAGE_2>    Sets the second image to use
```

# Benchmark
The below benchmarks have been done on a 2020 M1 Mac Mini with 16gb of ram and 256gb ssd, `hyperfine -w 3` is used and that command is then ran 3 times (outside of -w 3) and the result of the third run has been noted.

`baseline.jpeg` and `baseline_by_another_name.jpeg` should be identical, apart from the name.

`modified.jpeg` has been modified so that the very last (bottom right) pixel is red.


### hyperfine -w 3 'target/release/imeq images/baseline.jpeg images/baseline.jpeg'
```bash
  Time (mean ± σ):       1.0 ms ±   0.3 ms    [User: 0.6 ms, System: 0.2 ms]
  Range (min … max):     0.6 ms …   4.0 ms    936 runs
```

### hyperfine -w 3 'target/release/imeq images/baseline.jpeg images/baseline_by_another_name.jpeg'
```bash
  Time (mean ± σ):       5.3 ms ±   0.4 ms    [User: 7.3 ms, System: 2.2 ms]
  Range (min … max):     4.8 ms …   8.3 ms    384 runs
```

### hyperfine -w 3 'target/release/imeq images/baseline.jpeg images/modified.jpeg'
```bash
  Time (mean ± σ):     112.5 ms ±   2.1 ms    [User: 467.5 ms, System: 52.2 ms]
  Range (min … max):   110.4 ms … 121.6 ms    26 runs
```

## Credits
[Goat Image](https://unsplash.com/photos/J9wZ6D2kYPw) by [Florian van Duyn](https://unsplash.com/@flovayn)
