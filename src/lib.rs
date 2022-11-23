use pyo3::prelude::*;
use std::collections::HashMap;
use std::process::Command;

fn get_filename(filepath: &str, file_format: &str) -> String {
    let filename: Vec<&str> = filepath.split('.').collect();
    let after_filename = format!("{}.{}", filename[0], file_format);

    after_filename
}

fn parse_options(options: &HashMap<String, usize>) -> String {
    let mut option_vec: Vec<String> = Vec::new();

    if let Some(&val) = options.get("qv") {
        let opt = format!(" -q:v {}", val);
        option_vec.push(opt)
    }

    if let Some(&val) = options.get("ab") {
        let opt = format!(" -ab {}", val);
        option_vec.push(opt)
    }

    let mut option_arg = String::from("");

    for i in option_vec.iter() {
        option_arg += i
    }

    option_arg
}

fn ffmpeg_cmd(
    filepath: &str,
    file_format: &str,
    options: Option<&HashMap<String, usize>>,
) -> String {
    let mut option_arg = String::new();

    if let Some(val) = options {
        option_arg = parse_options(val);
    }

    let after_filename = get_filename(filepath, file_format);
    let cmd = format!("ffmpeg -y -i {}{} {}", filepath, option_arg, after_filename);

    cmd
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_filename() {
        assert_eq!(get_filename("test.mp4", "mp3"), "test.mp3")
    }

    #[test]
    fn test_ffmpeg_cmd_1() {
        assert_eq!(
            ffmpeg_cmd("test.mp4", "mp3", None),
            "ffmpeg -i test.mp4 test.mp3"
        )
    }

    #[test]
    fn test_ffmpeg_cmd_2() {
        let mut options: HashMap<String, usize> = HashMap::default();
        options.insert("qv".to_string(), 1);

        assert_eq!(
            ffmpeg_cmd("test.mp4", "mp3", Some(&options)),
            "ffmpeg -i test.mp4 test.mp3 -q:v 1"
        )
    }

    #[test]
    fn test_parse_options() {
        let mut options: HashMap<String, usize> = HashMap::default();
        options.insert("qv".to_string(), 1);

        assert_eq!(parse_options(&options), " -q:v 1")
    }
}
