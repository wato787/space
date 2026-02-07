# code

最小再現・サンプル置き場。

- `expressions-minimal/`: provider不要の最小構成
- `null-resource-triggers/`: null providerで `for_each` と `triggers` を確認

共通手順:

```bash
cd <example-dir>
terraform init
terraform plan
```

`apply` を行うとローカルstateや `.terraform.lock.hcl` が生成されるため、必要なときのみ実行する。

