# Ferris-Shooting

作: TunaMayoDX4

---

## 概要

WebGPUライブラリを中心とした、Rust製ライブラリを利用して作った、
Rust言語によるプログラミング、およびグラフィクスプログラミングの学習のために作成した、
簡単な2Dシューティングゲームです。

これは、Rustの非公式マスコットであるカニの"Ferris"くんちゃんが大量の歯車を発射し、苦労して作ったプログラムをドンドコ汚しながら踏み荒らす汚い例外をやっつけるという題材のゲームとなっております。

一般的な縦スクロールシューティングではありますが、現時点ではまだゲームオーバーの概念がなく、クリア・失敗ともにしない為、ストレスなくどんどんエラーを破壊しつくす、 **『お客様は神様』** という現代的社会に大変マッチした素晴らしいゲームとなっておりますことをここに宣言いたします。

嘘です。いずれゲームオーバーなりクリアなりするようにする予定ですので、よろしくお願いいたします。  
こちらの敗北については、主人公に敵が当たったら死ぬタイプではなく、取りこぼすと(画面内で敵を撃破しきれないと)どんどんダメージが蓄積していく方式にするつもりなので、敵をもう少し少なくするか、こっちの攻撃がガンガン当たるようにするつもりです。

---

## 免責

基本的にこれらのプログラムは弊環境で実際にコンパイル・テストし、メモリ・リークをはじめとしたリソース・リークが存在しないことを確認してからGitHub上に公開しておりますが、
勿論これらも完璧なものである保証はなく、また環境に依存、もしくは定義されていない操作を受け付けた場合に、不正な動作をしない保証は出来ないため、そこはご理解いただければと思います。

また、もしそれらが部分的にでも難しい場合には、閲覧・ダウンロード・コンパイル・実行等のいかなる形であれ、お避け頂ければと思います。

また、基本的に現時点ではウィンドウがアクティブの間は、パソコンのマウスによる操作を奪うため、強制的にフォア・グラウンドのウィンドウの操作権限を奪いうる方法を存じない場合にはダウンロード・コンパイル・実行のいずれもされないことをお勧めいたします。

---

## 使用方法

1. Rustコンパイラをインストールしてください。
2. 当リポジトリをクローンしてください。
3. コマンドプロンプトなどで、リポジトリのクローン先のディレクトリに移動し、適当に`cargo run --release`と入力してください。
4. 多分動きます。動かなかったら頑張ってください。

---

## 操作方法

### マウス

- マウス移動: カーソルの移動、照準
- 左クリック: 射撃
- 右クリック: 自動エイム・未来位置計算・表示
    - 未来位置は自分が選択している兵装の標準速度から計算されます。

### キーボード

- W/S: 前進/後退
- A/D: 左/右移動
- Z/C: 兵装切り替え
- Space: 射撃
- P: ポーズ／ポーズ解除
    - ポーズ中はマウス操作が可能になります。
- Escape(長押し): プログラムの終了