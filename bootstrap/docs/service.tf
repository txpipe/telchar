resource "kubernetes_service" "this" {
  metadata {
    name      = local.name
    namespace = var.namespace
    labels    = local.labels

  }

  spec {
    selector = local.labels
    type     = "ClusterIP"

    port {
      port        = local.port
      target_port = local.port
      protocol    = "TCP"
    }
  }
}
