use std::ptr::null;

use async_graphql::{connection::{Edge, EmptyFields}, types::connection::{query, Connection}, Error, Object, ID};

use crate::schema::pagination::AdditionalInfo;

use super::Scope;

#[derive(Default)]
pub struct ScopeQuery;

fn get_scopes() -> Vec<Scope> {
    vec![
        Scope::new(
            Some("e9e1585b-1e1d-470f-9335-0dcfa7652e30".to_string()),
            "Cardano",
            "https://github.com/cardanoscan",
        ),
        Scope::new(
            Some("c0778651-1b16-479a-878a-365193e8f89d".to_string()),
            "Minswap",
            "https://github.com/minswap",
        ),
        Scope::new(
            Some("94b91e56-1833-4f19-b296-5117d8eee5b0".to_string()),
            "NuFi",
            "https://github.com/nufi",
        ),
    ]
}

pub fn get_scope(id: ID) -> Option<Scope> {
    get_scopes().into_iter().find(|scope| scope.id == id)
}

#[Object]
impl ScopeQuery {
    async fn scopes(
        &self,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<usize, Scope, AdditionalInfo>, Error> {
        let scopes = get_scopes();
        query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let mut start = 0usize;
                let mut end = scopes.len();
    
                // Get first position of Scopes
                if let Some(after) = after {
                    if after >= end {
                        return Ok(Connection::with_additional_fields(false, false, AdditionalInfo::empty()));
                    }
                    start = after + 1;
                }
    
                // Get Last position of Scopes
                if let Some(before) = before {
                    if before == 0 {
                        return Ok(Connection::with_additional_fields(false, false, AdditionalInfo::empty()));
                    }
                    end = before;
                }
    
                // Get the slice of Scopes based on the initial and final positions
                let mut slice = &scopes[start..end];
    
                // Get the first N elements
                if let Some(first) = first {
                    slice = &slice[..first.min(slice.len())];
                    end -= first.min(slice.len());
                // Get the last N elements
                } else if let Some(last) = last {
                    slice = &slice[slice.len() - last.min(slice.len())..];
                    start = end - last.min(slice.len());
                }
    
                // Prepare the nodes based on calculated values
                let has_next_page = if let Some(first) = first {
                    slice.len() == first as usize && end < scopes.len()
                } else {
                    end < scopes.len()
                };

                let mut connection = Connection::with_additional_fields(
                    start > 0,
                    has_next_page,
                    AdditionalInfo::new(scopes.len(), slice.len()),
                );
                connection.edges.extend(
                    slice.iter().enumerate().map(|(idx, scope)| Edge::new(start + idx, (*scope).clone()))
                );
    
    
                Ok::<_, Error>(connection)
            }
        )
        .await
    }


    async fn scope(&self, id: ID) -> Option<Scope> {
        get_scope(id)
    }
}
