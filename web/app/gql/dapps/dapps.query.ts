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
