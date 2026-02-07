# notes

調査ログ（時系列メモ）用。

## 2026-02-07

- Terraformは宣言的で、差分計算はstateに対して行われる。stateの保全が最重要
- 既存リソースをTerraform管理に入れるときは `import` を使う。手動変更はdriftとして検知
- providerやmoduleはバージョン固定し、CIで `fmt` / `validate` を回す
- `for_each` はmap/set向きで、`count` は順序変化に弱い

