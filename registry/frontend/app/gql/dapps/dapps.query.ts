import { gql } from 'graphql-request';

export const DAPPS_QUERY = gql`
  query dapps {
    dapps {
      nodes {
        id
        name
        scope
        blueprint {
          version
        }
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
  query dapp($scope: String!, $name: String!) {
    dapp(scope: $scope, name: $name) {
      id
      name
      scope
      publishedDate
      repositoryUrl
      blueprintUrl
      readme
      blueprint {
        description
        version
        license
        compilerName
        compilerVersion
        plutusVersion
        validators {
          name
          datum {
            name
            schemaName
          }
          redeemer {
            name
            schemaName
          }
          parameters {
            name
            schemaName
          }
        }
      }
    }
  }
`;
