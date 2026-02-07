variable "items" {
  type        = map(string)
  description = "作成対象のキーとバージョン"
  default = {
    api    = "v1"
    worker = "v1"
  }
}

variable "env" {
  type        = string
  description = "環境名"
  default     = "dev"
}

variable "owner" {
  type        = string
  description = "責任者ラベル"
  default     = "learning"
}
