import { gql } from 'graphql-request';

export const DAPPS_DEFAULT_PAGINATION = {
  size: 15,
  page: 1,
};

export const DAPPS_QUERY = gql`
  query dapps($first: Int, $after: String, $last: Int, $before: String) {
    dapps(first: $first, last: $last, after: $after, before: $before) {
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

export function dappsQueryKeyGenerator(page = DAPPS_DEFAULT_PAGINATION.page, size = DAPPS_DEFAULT_PAGINATION.size) {
  return ['dapps', `page-${page}`, `size-${size}`];
}

export function generateDAppsArgs(page = DAPPS_DEFAULT_PAGINATION.page, size = DAPPS_DEFAULT_PAGINATION.size) {
  const after = ((page - 1) * size) - 1;
  return {
    first: size,
    after: after <= 0 ? null : after.toString(),
    before: null,
    last: null,
  };
}

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
        schemas {
          name
          schema
        }
      }
    }
  }
`;

export function dappQueryKeyGenerator(id: string) {
  return ['dapp', id];
}
