resource "kubernetes_ingress_v1" "this" {
  metadata {
    name      = local.name
    namespace = var.namespace
    annotations = {
      "cert-manager.io/cluster-issuer" = var.cert_issuer
    }
  }

  spec {
    ingress_class_name = "demeter"

    rule {
      host = var.hostname
      http {
        path {
          path = "/"
          backend {
            service {
              name = local.name
              port {
                number = local.port
              }
            }
          }
        }
      }
    }

    tls {
      hosts = [
        var.hostname
      ]
      secret_name = local.cert_secret
    }
  }
}
