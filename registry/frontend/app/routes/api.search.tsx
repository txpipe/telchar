// GQL
import {
  DAPPS_QUERY,
  dappsQueryKeyGenerator,
  generateDAppsArgs,
  DAPPS_DEFAULT_PAGINATION,
} from '~/gql/dapps/dapps.query';
import { requestGraphQL } from '~/gql/gql.server';

import type { Route } from './+types/api.search';

export async function loader({ context, ...others }: Route.LoaderArgs) {
  const url = new URL(others.request.url);
  const search = url.searchParams.get('q') || '';
  const result = await context.queryClient.fetchQuery({
    queryKey: dappsQueryKeyGenerator(1, DAPPS_DEFAULT_PAGINATION.size, search),
    queryFn: requestGraphQL<{ dapps: Query['dapps']; }, QueryDappsArgs>(
      DAPPS_QUERY,
      generateDAppsArgs(1, DAPPS_DEFAULT_PAGINATION.size, search),
    ),
  });

  return {
    dapps: result.dapps,
  };
}
