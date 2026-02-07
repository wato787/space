# checklist

テスト時に確認したいポイントのメモ。

## expressions-minimal

- `terraform plan` で `service_map` と `merged_tags` が期待どおりに展開される
- `terraform console` で `local.name_prefix` が `tf-dev` になる

## null-resource-triggers

- `terraform plan` で `null_resource.item` が `for_each` の件数分作成される
- `-var='env=staging'` で `triggers.env` が変わり、再作成が発生する
