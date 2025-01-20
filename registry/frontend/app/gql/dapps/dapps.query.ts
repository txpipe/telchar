import { gql } from 'graphql-request';

export const DAPPS_QUERY = gql`
  query dapps {
    dapps {
      nodes {
        id
        name
        scope
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
  query dapp($id: ID!) {
    dapp(id: $id) {
      id
      name
      scope
      description
      repository
      publishedDate

      plutusVersion
      version
      license
      compilerName
      compilerVersion
    }
  }
`;
