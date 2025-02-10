resource "kubernetes_ingress" "this" {
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
            service_name = local.name
            service_port = local.port
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
