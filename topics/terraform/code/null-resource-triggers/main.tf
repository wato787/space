terraform {
  required_version = ">= 1.5.0"

  required_providers {
    null = {
      source  = "hashicorp/null"
      version = ">= 3.2.1"
    }
  }
}

locals {
  common_tags = {
    env   = var.env
    owner = var.owner
  }
}

resource "null_resource" "item" {
  for_each = var.items

  triggers = merge(
    local.common_tags,
    {
      key   = each.key
      value = each.value
    }
  )
}
