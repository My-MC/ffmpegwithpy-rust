# ffmpegwithpy

This registry is a Rust rewrite of [ffmpegwithpy](https://github.com/My-MC/ffmpegwithpy).

## to Develop

### Install Dependence

``` bash
pip install maturin taskipy
```

### Develop Build

``` bash
task dev
```

### Test

```
# Run Python library test
pytest

# Run Rust library test
cargo test
```

### Build Release Wheel

``` bash
task build
```
