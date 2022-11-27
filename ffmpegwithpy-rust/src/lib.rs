use ffmpegwithpy_core::{ffmpeg_cmd, get_filename};
use pyo3::prelude::*;
use std::collections::HashMap;
use std::process::Command;

#[pyfunction]
#[pyo3(name = "get_filename")]
fn python_get_filename(filepath: &str, file_format: &str) -> PyResult<String> {
    let after_filename = get_filename(filepath, file_format);

    Ok(after_filename)
}

#[pyclass]
struct FFmpeg {
    filepath: String,
    file_format: String,
    options: Option<HashMap<String, usize>>,
}

#[pymethods]
impl FFmpeg {
    #[new]
    fn new(filepath: String, file_format: String, options: Option<HashMap<String, usize>>) -> Self {
        FFmpeg {
            filepath,
            file_format,
            options,
        }
    }

    #[pyo3(name = "ffmpeg_cmd")]
    fn python_ffmpeg_cmd(&self) -> PyResult<String> {
        let cmd: String = match &self.options {
            Some(val) => ffmpeg_cmd(&self.filepath, &self.file_format, Some(val)),
            None => ffmpeg_cmd(&self.filepath, &self.file_format, None),
        };

        Ok(cmd)
    }

    fn ffmpeg(&self) {
        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .arg("/C")
                .arg(ffmpeg_cmd(&self.filepath, &self.file_format, None))
                .output()
                .expect("ffmpeg command failed to start");
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(ffmpeg_cmd(&self.filepath, &self.file_format, None))
                .output()
                .expect("ffmpeg command failed to start");
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn ffmpegwithpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(python_get_filename, m)?)?;
    m.add_class::<FFmpeg>()?;
    Ok(())
}
