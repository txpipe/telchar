use async_graphql::{connection::Edge, types::connection::{query, Connection}, Error, Object, ID};

use super::Team;

#[derive(Default)]
pub struct TeamQuery;

fn get_teams() -> Vec<Team> {
    vec![
        Team::new(
            Some("e9e1585b-1e1d-470f-9335-0dcfa7652e30".to_string()),
            "Cardano",
            "https://github.com/cardanoscan",
        ),
        Team::new(
            Some("c0778651-1b16-479a-878a-365193e8f89d".to_string()),
            "Minswap",
            "https://github.com/minswap",
        ),
        Team::new(
            Some("94b91e56-1833-4f19-b296-5117d8eee5b0".to_string()),
            "NuFi",
            "https://github.com/nufi",
        ),
    ]
}

pub fn get_team(id: ID) -> Option<Team> {
    get_teams().into_iter().find(|team| team.id == id)
}

#[Object]
impl TeamQuery {
    async fn teams(
        &self,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<usize, Team>, Error> {
        let teams = get_teams();
        query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let mut start = 0usize;
                let mut end = teams.len();

                // Get first position of Teams
                if let Some(after) = after {
                    if after >= end {
                        return Ok(Connection::new(false, false));
                    }
                    start = after + 1;
                }

                // Get Last position of Teams
                if let Some(before) = before {
                    if before == 0 {
                        return Ok(Connection::new(false, false))
                    }
                    end = before;
                }

                // Get the slice of Teams based on the initial and final positions
                let mut slice = &teams[start..end];

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
                let mut connection: Connection<usize, Team, _, _, _, _, _> = Connection::new(start > 0, end < teams.len());
                connection.edges.extend(
                    slice.iter().enumerate().map(|(idx, team)| Edge::new(start + idx, (*team).clone()))
                );

                Ok::<_, Error>(connection)
            }
        )
        .await
    }

    async fn team(&self, id: ID) -> Option<Team> {
        get_team(id)
    }
}
