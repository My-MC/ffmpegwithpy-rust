# ffmpegwithpy

---

**ドキュメント**: <a href="https://my-mc.github.io/ffmpegwithpy-rust/" target="_blank">https://my-mc.github.io/ffmpegwithpy-rust/</a>

**ソースコード**: <a href="https://github.com/my-mc/ffmpegwithpy-rust" target="_blank">https://github.com/my-mc/ffmpegwithpy-rust</a>

---

ffmpegwithpyはFFmpegをPython上から実行するための、FFmpegラッパーです。

主な特徴:

* クラス構造による、わかりやすい設計
* 辞書型でオプションを指定できる

## インストール要件

Python 3.8+

!!! warning "注意"

    Python3.6以前はすでにEOLを迎えておりサポートしていません。

## インストール

!!! warning "注意"

    申し訳ございませんがただいまビルド済みのwheelは公開しておりません。
    手元での手動ビルドによるwheelをお使いください。

## Example

簡単なコード例を紹介します。

```python
from ffmpegwithpy import FFmpeg
process = FFmpeg("helloworld.mp4", "mp3")
# To run ffmpeg
process.ffmpeg()
# Get command of process
cmd = process.ffmpeg_cmd()
print(cmd)
```

## ライセンス

このライブラリはMITライセンスです。
