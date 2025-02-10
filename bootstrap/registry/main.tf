locals {
  service_account = "telchar-registry"
  name            = "telchar-registry"
  labels = {
    role : local.name
  }
  api_port         = 5000
  cert_name        = "telchar-registry"
  certs_mount_path = "/etc/certs"
  api_keys_secret  = "aws-keys"
}

provider "aws" {
  region = var.aws_region
}

variable "namespace" {
  type    = string
  default = "telchar"
}

variable "aws_region" {
  type    = string
  default = "us-west-2"
}

variable "aws_bucket_name" {
  type = string
}

variable "aws_dynamodb_table" {
  type    = string
  default = "ZotBlobTable"
}

variable "replicas" {
  type    = number
  default = 2
}

variable "image" {
  type = string
}

variable "resources" {
  type = object({
    limits = object({
      cpu    = string
      memory = string
    })
    requests = object({
      cpu    = string
      memory = string
    })
  })
  default = {
    limits : {
      cpu : "2",
      memory : "512Mi"
    }
    requests : {
      cpu : "50m",
      memory : "512Mi"
    }
  }
}

variable "tolerations" {
  type = list(object({
    effect   = string
    key      = string
    operator = string
    value    = optional(string)
  }))
  default = [
    {
      effect   = "NoSchedule"
      key      = "demeter.run/compute-profile"
      operator = "Equal"
      value    = "general-purpose"
    },
    {
      effect   = "NoSchedule"
      key      = "demeter.run/compute-arch"
      operator = "Equal"
      value    = "arm64"
    },
    {
      effect   = "NoSchedule"
      key      = "demeter.run/availability-sla"
      operator = "Equal"
      value    = "consistent"
    }
  ]
}

variable "hostname" {
  type = string
}

variable "cert_issuer" {
  type    = string
  default = "letsencrypt-dns01-cloudflare"
}
