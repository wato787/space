# null-resource-triggers

null providerを使って `for_each` と `triggers` の再作成挙動を確認する。

## 使い方

```bash
terraform init
terraform plan
```

## 変更確認

```bash
terraform plan -var='env=staging'
terraform plan -var='items={api="v2",worker="v1"}'
```

## 後片付け

```bash
terraform destroy -auto-approve
```
