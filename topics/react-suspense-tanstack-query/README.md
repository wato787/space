# react-suspense-tanstack-query

## 結論（TL;DR）

- TanStack Query を Suspense で使うと、読み込み中 UI を境界の `fallback` に集約できる
- v5 は `useSuspenseQuery` / `useSuspenseInfiniteQuery`、v4 は `useQuery` + `suspense: true`
- 再フェッチ時の再サスペンドを避けたい場合は `staleTime` や prefetch を調整する

## 目的 / 背景

- Suspense ベースのデータ取得フローとキャッシュ戦略の噛み合わせを理解する
- ローディング/エラーの責務分担（Suspense / ErrorBoundary / UI）を整理する

## メモ

- 詳細な調査ログは `notes.md` に残す
- Suspense 利用時は `isPending` などの状態分岐を減らして境界で切る

## コード例

- `code/README.md` に最小サンプルを置く
- 実行方法:
  - 既存の React + TanStack Query 環境に貼り付ける想定（このリポジトリ単体では実行しない）

## テスト

- 今回はドキュメント中心。実装時は ErrorBoundary と再フェッチ時の挙動を手動確認する

## links

- https://tanstack.com/query/latest/docs/framework/react/guides/suspense
- https://tanstack.com/query/latest/docs/framework/react/reference/useSuspenseQuery
- https://react.dev/reference/react/Suspense

## TODO

- v4/v5 の API 差分と移行ポイントを整理
- ErrorBoundary の選択肢（react-error-boundary など）を比較
- React Router の data API との組み合わせを試す

