# concepts

Terraformの基本用語を簡単に整理する。

## コア概念

- **configuration**: `.tf` で書く設定ファイル一式
- **provider**: 外部APIとの橋渡し（AWS/GCP/Nullなど）
- **resource**: 作成・更新・削除する対象
- **data source**: 参照専用の外部データ
- **module**: 設定のまとまり（root module / child module）
- **state**: リソースの実体とTerraformの対応表

## 実行フロー

- `init`: providerやbackendを初期化
- `plan`: 現在のstateと設定差分を表示
- `apply`: 差分を反映しstate更新
- `destroy`: リソース削除

## state / backend

- **local state**: ファイルで管理。小規模なら手軽
- **remote backend**: 共有・ロック・監査に強い
- **lock**: 競合防止のために必須
- **workspace**: 環境切り替えの単位

## 依存関係とライフサイクル

- 参照関係で暗黙的に依存が構築される
- `depends_on` で明示的依存を追加できる
- `lifecycle` で `create_before_destroy` / `prevent_destroy` を制御

## 変数と式

- `variable` / `locals` / `output` の役割を分ける
- `for_each` はmap/set向き、`count` は順序変化に注意
