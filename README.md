# lounge-memo

## TODO

- ステート遷移をちゃんと考える
  - エラーがはっせいしました の対応
- Detectorがうまく動かなかった時にもちゃんと動くようにしたい
  - CourseDetectorが失敗してもRaceFinishDetectorに移る、など
  - dyn Detector に対して実装？
- 認識しないコースがある
  - 例
    - どうぶつの森
    - 3DS レインボーロード
    - 3DS DKジャングル
    - アテネ
  - 認識方法を変える
    - 文字列の合否割合をとって許容度を上げる
- まともなフォーマットでテキストに書き出す
- デバッグを容易にできるようにする
  - PC内のmp4をデバッグに使いたい
- スプシ連携したい


## test

```
vcpkg install ffmpeg:x64-windows
```
