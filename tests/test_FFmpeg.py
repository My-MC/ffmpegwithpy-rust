import pytest
from ffmpegwithpy import FFmpeg, get_filename


@pytest.mark.parametrize(
    "filepath, file_format, cmd",
    [("get_filename.mp4", "mp3", "get_filename.mp3"), ("ゲット_ファイルネーム.webm", "mp4", "ゲット_ファイルネーム.mp4")],
)
def test_get_filename(filepath, file_format, cmd):
    assert get_filename(filepath, file_format) == cmd


@pytest.mark.parametrize(
    "filepath, file_format, options, cmd",
    [
        ("get_ffmpeg_cmd.mp4", "mp3", None, "ffmpeg -y -i get_ffmpeg_cmd.mp4 get_ffmpeg_cmd.mp3"),
        ("get_ffmpeg_cmd.mp4", "mp3", {"qv": 1}, "ffmpeg -y -i get_ffmpeg_cmd.mp4 -q:v 1 get_ffmpeg_cmd.mp3"),
        ("get_ffmpeg_cmd.mp4", "mp3", {"qv": 12}, "ffmpeg -y -i get_ffmpeg_cmd.mp4 -q:v 12 get_ffmpeg_cmd.mp3"),
        (
            "get_ffmpeg_cmd.mp4",
            "mp3",
            {"qv": 12, "ab": 192},
            "ffmpeg -y -i get_ffmpeg_cmd.mp4 -q:v 12 -ab 192 get_ffmpeg_cmd.mp3",
        ),
    ],
)
def test_get_ffmpeg_cmd(filepath, file_format, options, cmd):
    assert FFmpeg(filepath, file_format, options).ffmpeg_cmd() == cmd
