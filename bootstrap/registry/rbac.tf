resource "aws_iam_policy" "telchar_registry_policy" {
  depends_on = [
    aws_s3_bucket.this,
  ]

  name        = "TelcharRegistryPolicy"
  path        = "/"
  description = "Allows full access to Telchar S3 Bucket."
  policy = jsonencode({
    Version = "2012-10-17",
    Statement = [
      {
        Effect = "Allow",
        Action = [
          "s3:GetBucketLocation",
          "s3:ListBucket",
          "s3:ListBucketMultipartUploads"
        ],
        Resource = [
          aws_s3_bucket.this.arn,
        ],
      },
      {
        Effect = "Allow",
        Action = [
          "s3:AbortMultipartUpload",
          "s3:DeleteObject",
          "s3:GetObject",
          "s3:ListMultipartUploadParts",
          "s3:PutObject"
        ],
        Resource = [
          "${aws_s3_bucket.this.arn}/*",
        ],
      },
      {
        Effect = "Allow"
        Action = [
          "dynamodb:BatchGetItem",
          "dynamodb:BatchWriteItem",
          "dynamodb:CreateTable",
          "dynamodb:DeleteItem",
          "dynamodb:DescribeTable",
          "dynamodb:GetItem",
          "dynamodb:Scan",
          "dynamodb:UpdateItem"
        ]
        Resource : "*"
      }
    ],
  })
}

resource "aws_iam_user" "telchar_registry" {
  name = local.name
}

resource "aws_iam_user_policy_attachment" "this" {
  user       = aws_iam_user.telchar_registry.name
  policy_arn = aws_iam_policy.telchar_registry_policy.arn
}

resource "aws_iam_access_key" "telchar_registry" {
  user = aws_iam_user.telchar_registry.name
}
