# tests

学びを固定するためのテスト置き場。

## 前提

- Terraform CLIがインストール済み

## 手動チェック（例）

```bash
cd topics/terraform

# 例1: expressions-minimal
terraform -chdir=code/expressions-minimal fmt -check
terraform -chdir=code/expressions-minimal init -input=false
terraform -chdir=code/expressions-minimal validate

# 例2: null-resource-triggers
terraform -chdir=code/null-resource-triggers fmt -check
terraform -chdir=code/null-resource-triggers init -input=false
terraform -chdir=code/null-resource-triggers validate
```

- 期待する挙動は `checklist.md` に整理する

