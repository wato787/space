variable "environment" {
  type        = string
  description = "環境名（例: dev/staging/prod）"
  default     = "dev"
}

variable "service_names" {
  type        = list(string)
  description = "サービス名の一覧"
  default     = ["api", "worker", "web"]
}

variable "enable_feature" {
  type        = bool
  description = "機能フラグ"
  default     = true
}

variable "tags" {
  type        = map(string)
  description = "共通タグ"
  default = {
    owner = "learning"
  }
}
