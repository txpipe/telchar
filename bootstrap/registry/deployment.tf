resource "kubernetes_deployment_v1" "telchar_registry" {
  wait_for_rollout = false
  depends_on       = [kubernetes_manifest.certificate_cluster_wildcard_tls]

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
    strategy {
      rolling_update {
        max_surge       = 2
        max_unavailable = 0
      }
    }
    template {
      metadata {
        name   = local.name
        labels = local.labels
      }
      spec {
        container {
          name              = "main"
          image             = var.image
          image_pull_policy = "IfNotPresent"

          resources {
            limits = {
              cpu    = var.resources.limits.cpu
              memory = var.resources.limits.memory
            }
            requests = {
              cpu    = var.resources.requests.cpu
              memory = var.resources.requests.memory
            }
          }

          port {
            name           = "api"
            container_port = local.api_port
            protocol       = "TCP"
          }

          env {
            name = "AWS_ACCESS_KEY_ID"
            value_from {
              secret_key_ref {
                key  = "aws_access_key_id"
                name = local.api_keys_secret
              }
            }
          }

          env {
            name = "AWS_SECRET_ACCESS_KEY"
            value_from {
              secret_key_ref {
                key  = "aws_secret_access_key"
                name = local.api_keys_secret
              }
            }
          }

          volume_mount {
            mount_path = "/etc/zot/"
            name       = "config"
          }

          volume_mount {
            mount_path = local.certs_mount_path
            name       = "certs"
          }
        }

        volume {
          name = "config"
          config_map {
            name = kubernetes_config_map.config.metadata.0.name
          }
        }

        volume {
          name = "certs"
          secret {
            secret_name = local.cert_name
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
