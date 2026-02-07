output "name_prefix" {
  value       = local.name_prefix
  description = "環境を含んだプレフィックス"
}

output "service_map" {
  value       = local.service_map
  description = "サービス名 -> リソース名の対応"
}

output "merged_tags" {
  value       = local.merged_tags
  description = "統合済みタグ"
}

output "safe_owner" {
  value       = local.safe_owner
  description = "ownerタグの安全な参照"
}
