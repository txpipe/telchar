use async_graphql::{Object, ID};

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
    async fn teams(&self) -> Vec<Team> {
        get_teams()
    }

    async fn team(&self, id: ID) -> Option<Team> {
        get_team(id)
    }
}
