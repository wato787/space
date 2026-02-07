# terraform

## 結論（TL;DR）

- Terraformは宣言的IaCで、差分計算の基準は**state**。stateの保全とロックが最重要
- `plan`で差分確認→`apply`で反映が基本。手動変更はdriftとして検知し、`import`でstateに取り込む
- providerとmoduleは**バージョン固定**し、入力/出力と責務を明示する
- 環境分離はbackend + workspaceで行い、CIで`fmt/validate`を通す

## 目的 / 背景

- 仕事でTerraformを扱う機会が増えたため、基本概念と安全運用の要点を整理する
- 小さな例で「式/locals」「for_each」「依存関係」「state」を再確認する

## メモ

- 詳細な調査ログは `notes.md` に残す
- 用語整理: `concepts.md`
- CLIチートシート: `cheatsheet.md`

## コード例

- `code/expressions-minimal/`: provider不要の最小構成で式/locals/outputsを確認
- `code/null-resource-triggers/`: null providerで `for_each` と `triggers` を確認
- 実行方法:
  - Terraform CLIが必要（`terraform version` で確認）
  - 各ディレクトリで `terraform init` → `terraform plan`
  - `apply` はローカルstateを作るので必要なときのみ

## テスト

- `tests/README.md` に手動チェックとコマンド例を記載
- `terraform fmt -check` と `terraform validate` を基本ゲートにする

## links

- https://developer.hashicorp.com/terraform/docs
- https://developer.hashicorp.com/terraform/language
- https://developer.hashicorp.com/terraform/cli
- https://developer.hashicorp.com/terraform/language/state
- https://developer.hashicorp.com/terraform/language/backend
- https://registry.terraform.io/
- https://registry.terraform.io/providers/hashicorp/null/latest

## TODO

- remote backend（S3 + DynamoDB）の最小例を追加
- `import` / `moved` ブロックの例を追加
- `terraform test` の最小例を追加

