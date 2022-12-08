# ffmpegwithpy

---

**Documentation**: <a href="https://my-mc.github.io/ffmpegwithpy-rust/" target="_blank">https://my-mc.github.io/ffmpegwithpy-rust/</a>

**SourceCode**: <a href="https://github.com/my-mc/ffmpegwithpy-rust" target="_blank">https://github.com/my-mc/ffmpegwithpy-rust</a>

---

ffmpegwithpy is an FFmpeg wrapper to run FFmpeg from Python.

Main Features:

* Easy-to-understand design with class structure
* Dictionary-type options can be specified

## Installation Requirements

Python 3.7+

!!! warning

    Python 3.6 and earlier have already reached EOL and are not supported.

## Installation

!!! warning

    We are sorry, but we are not releasing pre-built wheels at this time.
    Please use wheels built manually.

## Example

Here is a simple code example.

```python
from ffmpegwithpy import FFmpeg
process = FFmpeg("helloworld.mp4", "mp3")
# To run ffmpeg
process.ffmpeg()
# Get command of process
cmd = process.ffmpeg_cmd()
print(cmd)
```

## License

This library is licensed under the MIT License.
