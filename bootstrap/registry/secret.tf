resource "kubernetes_secret" "aws_keys" {
  metadata {
    name      = local.api_keys_secret
    namespace = var.namespace
  }

  type = "Opaque"

  data = {
    aws_access_key_id     = aws_iam_access_key.telchar_registry.id
    aws_secret_access_key = aws_iam_access_key.telchar_registry.secret
  }
}
