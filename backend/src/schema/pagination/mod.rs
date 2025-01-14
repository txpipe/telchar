use async_graphql::SimpleObject;

#[derive(SimpleObject)]
struct PaginationInfo {
  total_nodes: usize,
  page_size: usize,
}

#[derive(SimpleObject)]
pub struct AdditionalInfo {
  metadata: Option<PaginationInfo>,
}

impl AdditionalInfo {
  pub fn new(total_nodes: usize, page_size: usize) -> Self {
    Self {
      metadata: Some(PaginationInfo { total_nodes, page_size })
    }
  }

  pub fn empty() -> Self {
    Self { metadata: None }
  }
}
