resource "kubernetes_service_v1" "telchar_lb" {
  depends_on = [kubernetes_namespace.namespace]

  metadata {
    name      = local.name
    namespace = var.namespace
    annotations = {
      "service.beta.kubernetes.io/aws-load-balancer-nlb-target-type" : "instance"
      "service.beta.kubernetes.io/aws-load-balancer-scheme" : "internet-facing"
      "service.beta.kubernetes.io/aws-load-balancer-type" : "external"
    }
  }

  spec {
    load_balancer_class = "service.k8s.aws/nlb"
    selector            = local.labels

    port {
      name        = "zot-registry"
      port        = 443
      target_port = local.api_port
      protocol    = "TCP"
    }

    type = "LoadBalancer"
  }
}
