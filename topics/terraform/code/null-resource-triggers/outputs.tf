output "item_ids" {
  value       = { for k, r in null_resource.item : k => r.id }
  description = "null_resourceのID一覧"
}
