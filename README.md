# lounge-memo

キャプチャーボードから取得できる画像から、どのコースを走り、何位だったのか？を自動でメモするためのツール

## Features

- レース結果を `results.txt` に出力
  - コース
  - 順位
  - 累計ポイント
- レース終了後にリザルトのスクリーンショットを保存
  - 各レースで何位を取ったか
  - 各レースの総合順位

## Environment

- Windows
  - tested on Windows 11

## Run

```cmd
cargo run --release -- [CAPTURE DEVICE NUMBER (default 0)]
```

## TODO

- ステート遷移をちゃんと考える
  - 現状は以下の3つがある
    - CourseDetector
    - RaceFinishDetector
    - PositionDetector
    - CaptureTotalScoresDetector
  - Finishが出てからPositionに行くことを考える
- Detectorがうまく動かなかった時にもちゃんと動くようにしたい
  - CourseDetectorが失敗してもRaceFinishDetectorに移る、など
  - dyn Detector に対して実装？
- 認識しないコースがある
  - どうにか外部から情報を修正する方法を用意する？
    - 右下にあるコースプレビューの一致度などから
    - 3DSなどのシリーズを表すprefixは画像の一致から取得する、など
- まともなフォーマットでテキストに書き出す
- スプシ連携したい
  - 方法案
    - プラグイン形式
      - wasm？
    - 別のプログラムが食えるようなjsonを書き出す
- コース編集時に略称をそのまま入れられるようになるべき
  - 補完とかせず、略称が期待するものと一致していればEnterキーで補完できてほしい
  - 一致しない場合は要検証
    - 何もしない
    - ボックスの中身を消す
  - 略称のソースは[ここ](https://github.com/sheat-git/mk8dx.py/blob/main/mk8dx/data.py)にある
- 即時もする
  - OCRが不安定なので微妙かも？
  - タグ間違いなどにどうやって対応するべきか

## test

```
vcpkg install ffmpeg:x64-windows
cargo test
```
