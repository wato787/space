# cheatsheet

よく使うCLIコマンドのメモ。

## 基本

```bash
terraform fmt
terraform fmt -check
terraform init -upgrade
terraform validate
terraform plan -out plan.tfplan
terraform show plan.tfplan
terraform apply plan.tfplan
terraform destroy
```

## state操作

```bash
terraform state list
terraform state show <resource_address>
terraform state mv <src> <dst>
terraform state rm <resource_address>
terraform import <resource_address> <id>
```

## 便利系

```bash
terraform console
terraform providers
terraform workspace list
terraform workspace new <name>
terraform workspace select <name>
terraform graph | dot -Tpng > graph.png
```
