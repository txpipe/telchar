use async_graphql::InputObject;

#[derive(InputObject)]
pub struct SearchDAppByScope {
    pub name: String,
    pub scope_name: String,
}