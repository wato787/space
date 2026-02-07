# expressions-minimal

provider不要の最小構成で、式/locals/outputsの挙動を確認する。

## 使い方

```bash
terraform init
terraform plan
```

## console例

```bash
terraform console
> local.name_prefix
> local.service_map
> local.merged_tags
```

## 期待結果

- `name_prefix` は `tf-dev`
- `service_map` は `api` / `worker` / `web` を含む
