imeq-rs aims to quickly compare two images to see if they are the same image.

## Installation
```
cargo install imeq
```

## Usage
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

## Building
Compiles to: `target/release/imeq`
```
cargo build --release
```

## Benchmark
The below benchmarks have been done on a 2020 M1 Mac Mini with 16gb of ram and 256gb ssd, `hyperfine -w 3` is used and that command is then ran 3 times (outside of -w 3) and the result of the third run has been noted.

`baseline.jpeg` and `baseline_by_another_name.jpeg` should be identical, apart from the name.

`flipped.jpeg` has been flipped horizontally.

`modified.jpeg` has been modified so that the very last (bottom right) pixel is red.



### hyperfine -w 3 'target/release/imeq images/baseline.jpeg images/baseline.jpeg'
```bash
  Time (mean ± σ):       1.1 ms ±   0.3 ms    [User: 0.6 ms, System: 0.3 ms]
  Range (min … max):     0.8 ms …   2.6 ms    857 runs
```

### hyperfine -w 3 'target/release/imeq images/baseline.jpeg images/baseline_by_another_name.jpeg'
```bash
  Time (mean ± σ):       5.5 ms ±   0.4 ms    [User: 7.4 ms, System: 2.3 ms]
  Range (min … max):     5.1 ms …   7.5 ms    308 runs
```

### hyperfine -w 3 'target/release/imeq images/baseline.jpeg images/flipped.jpeg'
```bash
  Time (mean ± σ):      93.7 ms ±   3.4 ms    [User: 443.6 ms, System: 54.2 ms]
  Range (min … max):    90.4 ms … 102.8 ms    31 runs
```

### hyperfine -w 3 'target/release/imeq images/baseline.jpeg images/modified.jpeg'
```bash
  Time (mean ± σ):     113.1 ms ±   0.9 ms    [User: 471.1 ms, System: 53.4 ms]
  Range (min … max):   111.4 ms … 115.5 ms    26 runs
```

## Credits

- [Handmade Web & Design](https://github.com/handmadeweb)
- [Michael Rook](https://github.com/michaelr0)
- [All Contributors](https://github.com/michaelr0/imeq-rs/graphs/contributors)

[Goat Image](https://unsplash.com/photos/J9wZ6D2kYPw) by [Florian van Duyn](https://unsplash.com/@flovayn)

## License

The MIT License (MIT). Please see [License File](https://github.com/michaelr0/imeq-rs/blob/main/LICENSE.md) for more information.
