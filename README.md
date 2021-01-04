# websocket_test_server
websocketの接続テストなどをするためのサーバーおよびクライアント実装です。

下記のリファレンス実装を元に、動作を明らかにするためにリファクタを加えた形です。
https://github.com/actix/examples/tree/master/websocket

Rustの勉強とwebsocketの実験用に作りました。両方ともテストに使いやすいように拡張していけるといいですね。

## リファクタ箇所
- エラーハンドリングの改善(unwrap less)
- リネーム
- 標準入力からMessage::Closeを作成してクライアント側からwsを通じて明示的に終了可能にする
- サーバーレスポンスをJSONっぽくして現実のサーバーに近づける
- cargoのコンフィグを使い、直観的に起動できるようにした

## how to use
※cargoが必要です。開発環境はarchlinux+rustupを使用しています。
```sh
cargo run --bin server # サーバーの起動
cargo run --bin client # クライアントの起動
```

ポートは8080を使用します。不都合があれば変えてください。(rustならCLIとしてカスタマイズするのも余裕ですが)

クライアントは標準入力を受け付けます。(例えばabcなど入力してEnterするとサーバーに送信できます。)

exitを入力して送信するとStop命令を発行してサーバーとの接続を終了します。

サーバー側はクライアント側の送信に対するecho処理と、5秒ごとに現在時刻を送信するperiodic処理を実装しています。これを応用すればwebsocketの基本的な処理は書けるはずです。
