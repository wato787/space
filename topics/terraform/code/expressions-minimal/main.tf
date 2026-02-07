terraform {
  required_version = ">= 1.5.0"
}

locals {
  env          = var.environment
  name_prefix  = "tf-${local.env}"
  safe_owner   = try(var.tags.owner, "unknown")
  feature_flag = var.enable_feature ? "enabled" : "disabled"

  normalized_services = [for s in var.service_names : lower(s)]

  merged_tags = merge(
    var.tags,
    {
      env     = local.env
      feature = local.feature_flag
    }
  )

  service_map = {
    for s in local.normalized_services :
    s => "${local.name_prefix}-${s}"
  }
}
