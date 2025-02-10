locals {
  name        = "telchar-frontend"
  port        = 3000
  cert_secret = "telchar-frontend"
  labels = {
    role : "telchar-frontend"
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
      cpu    = "250m"
      memory = "256Mi"
    }
    limits = {
      memory = "256Mi"
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
