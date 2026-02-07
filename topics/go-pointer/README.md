# go-pointer

## 結論（TL;DR）

- ポインタ型は `*T`。`&x` でアドレス取得、`*p` で参照/代入する
- ゼロ値は `nil`。参照前に `nil` チェックする
- 構造体の更新やコピー回避にはポインタを使う。スライス/マップ/チャネルは参照型なので通常はポインタ不要
- `new(T)` はゼロ値の `*T` を返す。`&T{}` はリテラルで初期化した `*T` を返す

## 目的 / 背景

- 値渡しと参照渡しの違いを明確にし、状態の更新やコピーのコストを理解する
- ループ変数のアドレス取得など、Go特有の落とし穴を避けられるようにする

## メモ

- 詳細な調査ログは `notes.md` に残す
- `range` ループ変数のアドレスを取ると同じ変数を共有してしまう（必要なら要素をローカル変数にコピー）

## コード例

- `code/` に基本操作をまとめたサンプルを置く
- 実行方法:
  - Go が必要（`go version`）
  - `cd code && go run .`

## テスト

- 手動チェックは `tests/README.md` に記載
- 追加で確認する場合: `cd code && go test ./...`

## links

- https://go.dev/tour/moretypes/1
- https://go.dev/ref/spec#Pointer_types
- https://go.dev/doc/effective_go#pointers
- https://go.dev/doc/faq#stack_or_heap

## TODO

- `range` 変数のアドレス取得の落とし穴サンプルを追加
- ポインタ受け取り vs 値受け取りの性能差の簡単なベンチを追加

