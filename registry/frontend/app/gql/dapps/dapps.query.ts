import { gql } from 'graphql-request';

export const DAPPS_DEFAULT_PAGINATION = {
  size: 15,
  page: 1,
};

export const DAPPS_QUERY = gql`
  query dapps($pageSize: Int, $offset: Int, $search: String) {
    dapps(pageSize: $pageSize, offset: $offset, search: $search) {
      nodes {
        id
        name
        scope
        version
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

export function dappsQueryKeyGenerator(
  page = DAPPS_DEFAULT_PAGINATION.page,
  size = DAPPS_DEFAULT_PAGINATION.size,
  search = '',
) {
  const _search = search.trim().toLowerCase();
  const output = ['dapps', `page-${page}`, `size-${size}`];
  if (_search.length > 0) {
    output.push(`search-${_search}`);
  }
  return output;
}

export function generateDAppsArgs(
  page = DAPPS_DEFAULT_PAGINATION.page,
  size = DAPPS_DEFAULT_PAGINATION.size,
  search = '',
): QueryDappsArgs {
  const after = ((page - 1) * size);
  const _search = search.trim().toLowerCase();
  return {
    pageSize: size,
    offset: after <= 0 ? null : after,
    search: _search.length > 0 ? _search : null,
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
