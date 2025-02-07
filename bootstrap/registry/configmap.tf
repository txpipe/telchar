resource "kubernetes_config_map" "config" {
  depends_on = [kubernetes_namespace.namespace]

  metadata {
    namespace = var.namespace
    name      = "telchar-config"
  }

  data = {
    "config.json" = jsonencode({
      storage : {
        rootDirectory : "/tmp/zot",
        dedupe : true
        storageDriver : {
          name : "s3"
          region : var.aws_region
          bucket : var.aws_bucket_name
          secure : true
          skipverify : false
        }
        cacheDriver : {
          name : "dynamodb"
          endpoint : "https://dynamodb.${var.aws_region}.amazonaws.com"
          region : var.aws_region
          cacheTablename : var.aws_dynamodb_table
          repoMetaTablename : "ZotRepoMetadataTable"
          manifestDataTablename : "ZotManifestDataTable"
          versionTablename : "ZotVersion"
          repoBlobsInfoTableName : "ZotRepoBlobsInfoTable"
          imageMetaTableName : "ZotImageMetaTable"
          apiKeyTableName : "ZotApiKeyTable"
          userDataTableName : "ZotUserDataTable"
        }
      }
      http : {
        address : "0.0.0.0",
        port : local.api_port,
        tls : {
          cert : "${local.certs_mount_path}/tls.crt",
          key : "${local.certs_mount_path}/tls.key"
        }
      }
      log : {
        level : "debug"
      }
      extensions : {
        search : {
          enable : true
        }
        ui : {
          enable : false
        }
        mgmt : {
          enable : true
        }
        metrics : {
          enable : true,
          prometheus : {
            path : "/metrics"
          }
        }
      }
    })
  }
}
