resource "kubernetes_manifest" "telchar_registry_monitor" {
  depends_on = [kubernetes_namespace.namespace]

  manifest = {
    apiVersion = "monitoring.coreos.com/v1"
    kind       = "PodMonitor"
    metadata = {
      labels = {
        "app.kubernetes.io/component" = "o11y"
        "app.kubernetes.io/part-of"   = "telchar"
      }
      name      = local.name
      namespace = var.namespace
    }
    spec = {
      selector = {
        matchLabels = local.labels
      }
      podMetricsEndpoints = [
        {
          port = "api",
          path = "/metrics"
        }
      ]
    }
  }
}
