// GQL
import {
  DAPPS_QUERY,
  dappsQueryKeyGenerator,
  generateDAppsArgs,
  DAPPS_DEFAULT_PAGINATION,
} from '~/gql/dapps/dapps.query';
import { requestGraphQL } from '~/gql/gql.server';

// Pages
import { Home as HomePage } from '~/pages/home';

import type { Route } from './+types/_index';

export function meta({}: Route.MetaArgs) {
  return [
    { title: 'Telchar' },
    { name: 'description', content: 'Telchar descriptions' },
  ];
}

export async function loader({ context, ...others }: Route.LoaderArgs) {
  const url = new URL(others.request.url);
  const page = parseInt(url.searchParams.get('page') || DAPPS_DEFAULT_PAGINATION.page.toString(), 10)
    || DAPPS_DEFAULT_PAGINATION.page;
  let size = parseInt(url.searchParams.get('size') || DAPPS_DEFAULT_PAGINATION.size.toString(), 10)
    || DAPPS_DEFAULT_PAGINATION.size;

  // Limit the size to 25
  if (size > 25) {
    size = 25;
  }

  const result = await context.queryClient.fetchQuery({
    queryKey: dappsQueryKeyGenerator(page, size),
    queryFn: requestGraphQL<{ dapps: Query['dapps']; }, QueryDappsArgs>(
      DAPPS_QUERY,
      generateDAppsArgs(page, size),
    ),
  });

  return {
    dapps: result.dapps,
  };
}

export default function Home({ loaderData }: Route.ComponentProps) {
  return <HomePage dapps={loaderData.dapps} />;
}
