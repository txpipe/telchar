resource "kubernetes_manifest" "certificate_cluster_wildcard_tls" {
  depends_on = [kubernetes_namespace.namespace]

  manifest = {
    "apiVersion" = "cert-manager.io/v1"
    "kind"       = "Certificate"
    "metadata" = {
      "name"      = local.cert_name
      "namespace" = var.namespace
    }
    "spec" = {
      "dnsNames" = [
        var.hostname

      ]

      "issuerRef" = {
        "kind" = "ClusterIssuer"
        "name" = "letsencrypt-http01"
      }
      "secretName" = local.cert_name
    }
  }
}
