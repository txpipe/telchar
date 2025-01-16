import { gql } from 'graphql-request';

export const DAPPS_QUERY = gql`
  query dapps {
    dapps {
      nodes {
        id
        name
        scope {
          id
          name
        }
        description
        publishedDate
      }
      pageInfo {
        hasPreviousPage
        hasNextPage
        startCursor
        endCursor
      }
      metadata {
        totalNodes
        pageSize
      }
    }
  }
`;

export const DAPP_QUERY = gql`
  query dapp($id: ID, $input: SearchDAppByScope) {
    dapp(id: $id, input: $input) {
      id
      name
      description
      repository
      publishedDate
      scopeId
      scope {
        id
        name
        repository
      }
    }
  }
`;
