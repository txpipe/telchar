locals {
  name        = "telchar-backend"
  port        = 8000
  cert_secret = "telchar-backend"
  labels = {
    role : "telchar-backend"
  }
}

variable "namespace" {
  type    = string
  default = "telchar"
}

variable "replicas" {
  type    = number
  default = 2
}

variable "image" {
  type = string
}

variable "registry_host" {
  type = string
}

variable "registry_protocol" {
  type = string
}

variable "cert_issuer" {
  type    = string
  default = "letsencrypt-dns01-cloudflare"
}

variable "ingress_class" {
  type    = string
  default = "demeter"
}

variable "hostname" {
  type = string
}

variable "resources" {
  type = object({
    requests = object({
      cpu    = optional(string)
      memory = string
    })
    limits = object({
      cpu    = optional(string)
      memory = string
    })
  })
  default = {
    requests = {
      cpu    = "100m"
      memory = "2Gi"
    }
    limits = {
      cpu    = "1"
      memory = "2Gi"
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
      value    = "x86"
    },
    {
      effect   = "NoSchedule"
      key      = "demeter.run/availability-sla"
      operator = "Equal"
      value    = "best-effort"
    }
  ]
}
