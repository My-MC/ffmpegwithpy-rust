use ffmpegwithpy_core::{ffmpeg_cmd, get_filename};
use maplit::hashmap;
use rstest::rstest;
use std::collections::HashMap;

#[rstest(
    filepath,
    file_format,
    result,
    case("get_filename.mp4", "mp3", "get_filename.mp3"),
    case("ゲット_ファイルネーム.webm", "mp4", "ゲット_ファイルネーム.mp4")
)]
fn test_get_filename(filepath: &str, file_format: &str, result: &str) {
    assert_eq!(get_filename(filepath, file_format), result);
}

#[test]
fn test_ffmpeg_cmd_1() {
    assert_eq!(
        ffmpeg_cmd("test.mp4", "mp3", None),
        "ffmpeg -y -i test.mp4 test.mp3"
    )
}

#[rstest(
    filepath,
    file_format,
    options,
    result,
    case("ffmpeg_cmd.mp4", "mp3", &hashmap! {"qv".to_string() => 1}, "ffmpeg -y -i ffmpeg_cmd.mp4 -q:v 1 ffmpeg_cmd.mp3"),
    case("ffmpeg_cmd.mp4", "mp3", &hashmap! {"qv".to_string() => 12}, "ffmpeg -y -i ffmpeg_cmd.mp4 -q:v 12 ffmpeg_cmd.mp3"),
    case("ffmpeg_cmd.mp4", "mp3", &hashmap! {"qv".to_string() => 12, "ab".to_string() => 192}, "ffmpeg -y -i ffmpeg_cmd.mp4 -q:v 12 -ab 192 ffmpeg_cmd.mp3")
)]
fn test_ffmpeg_cmd_2(
    filepath: &str,
    file_format: &str,
    options: &HashMap<String, usize>,
    result: &str,
) {
    assert_eq!(ffmpeg_cmd(filepath, file_format, Some(options)), result)
}
