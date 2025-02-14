import { gql } from 'graphql-request';

export const DAPPS_DEFAULT_PAGINATION = {
  size: 15,
  page: 1,
};

export const DAPPS_DEFAULT_SORT = 'alphabetical';

export const DAPPS_QUERY = gql`
  query dapps($pageSize: Int, $offset: Int, $search: String, $sortBy: DappSort) {
    dapps(pageSize: $pageSize, offset: $offset, search: $search, sortBy: $sortBy) {
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
  sortBy = DAPPS_DEFAULT_SORT,
) {
  const _search = search.trim().toLowerCase();
  const output = ['dapps', `page-${page}`, `size-${size}`, `sortBy-${sortBy}`];
  if (_search.length > 0) {
    output.push(`search-${_search}`);
  }
  return output;
}

export function generateDAppsArgs(
  page = DAPPS_DEFAULT_PAGINATION.page,
  size = DAPPS_DEFAULT_PAGINATION.size,
  search = '',
  sortBy = DAPPS_DEFAULT_SORT,
): QueryDappsArgs {
  const after = ((page - 1) * size);
  const _search = search.trim().toLowerCase();
  let _sortBy: DappSort = 'ALPHABETIC_ASC';
  switch (sortBy) {
    case 'most-recent':
      _sortBy = 'UPDATE_TIME';
      break;
    case 'most-viewed':
      _sortBy = 'DOWNLOADS';
      break;
    case 'alphabetical':
    default:
      _sortBy = 'ALPHABETIC_ASC';
      break;
  }
  return {
    pageSize: size,
    offset: after <= 0 ? null : after,
    search: _search.length > 0 ? _search : null,
    sortBy: _sortBy,
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

export const SMALL_DAPP_QUERY = gql`
  query dapp($scope: String!, $name: String!) {
    dapp(scope: $scope, name: $name) {
      id
      blueprintUrl
    }
  }
`;

export function dappQueryKeyGenerator(id: string) {
  return ['dapp', id];
}
