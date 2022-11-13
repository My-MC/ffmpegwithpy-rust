import pytest

from ffmpegwithpy import FFmpeg, get_filename


def test_get_filename():
    after_filename = get_filename("test.mp4", "mp3")
    assert after_filename == "test.mp3"


@pytest.mark.parametrize(
    "filepath, file_format, cmd",
    [
        ("test.mp4", "mp3", "ffmpeg -i test.mp4 test.mp3"),
        ("テスト.webm", "mp4", "ffmpeg -i テスト.webm テスト.mp4"),
    ],
)
def test_FFmpeg_ffmpeg_cmd(filepath, file_format, cmd):
    assert FFmpeg(filepath, file_format).ffmpeg_cmd() == cmd


@pytest.mark.parametrize(
    "filepath, file_format, options, cmd",
    [
        ("test.mp4", "mp3", {"qv": 1}, "ffmpeg -i test.mp4 test.mp3 -q:v 1"),
        ("テスト.webm", "mp4", {"qv": 10}, "ffmpeg -i テスト.webm テスト.mp4 -q:v 10"),
    ],
)
def test_FFmpeg_ffmpeg_cmd_with_option(filepath, file_format, options, cmd):
    assert FFmpeg(filepath, file_format, options).ffmpeg_cmd() == cmd
