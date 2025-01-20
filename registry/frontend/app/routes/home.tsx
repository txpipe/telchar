// GQL
import { DAPPS_QUERY } from '~/gql/dapps/dapps.query';
import { requestGraphQL } from '~/gql/gql.server';

// Pages
import { Home as HomePage } from '~/pages/home';

import type { Route } from './+types/home';

export function meta({}: Route.MetaArgs) {
  return [
    { title: 'Telchar' },
    { name: 'description', content: 'Telchar descriptions' },
  ];
}

export async function loader({ context }: Route.LoaderArgs) {
  const result = await context.queryClient.fetchQuery({
    queryKey: ['dapps'],
    queryFn: requestGraphQL<{ dapps: Query['dapps']; }>(DAPPS_QUERY),
  });

  return {
    initialDApps: result.dapps,
  };
}

export default function Home({ loaderData }: Route.ComponentProps) {
  return <HomePage initialDApps={loaderData.initialDApps} />;
}
