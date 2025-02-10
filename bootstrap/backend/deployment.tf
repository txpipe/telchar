resource "kubernetes_deployment" "this" {
  metadata {
    name      = local.name
    namespace = var.namespace
    labels    = local.labels
  }

  spec {
    replicas = var.replicas
    selector {
      match_labels = local.labels
    }

    template {
      metadata {
        name   = local.name
        labels = local.labels
      }

      spec {
        restart_policy = "Always"

        container {
          name              = "main"
          image             = var.image
          image_pull_policy = "IfNotPresent"

          env {
            name  = "REGISTRY_HOST"
            value = var.registry_host
          }

          env {
            name  = "REGISTRY_PROTOCOL"
            value = var.registry_protocol
          }

          resources {
            limits   = var.resources.limits
            requests = var.resources.requests
          }
        }

        dynamic "toleration" {
          for_each = var.tolerations

          content {
            effect   = toleration.value.effect
            key      = toleration.value.key
            operator = toleration.value.operator
            value    = toleration.value.value
          }
        }
      }
    }
  }
}
