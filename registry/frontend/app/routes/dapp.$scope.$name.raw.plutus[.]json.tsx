import { redirect } from 'react-router';

// GQL
import { SMALL_DAPP_QUERY, dappQueryKeyGenerator } from '~/gql/dapps/dapps.query';
import { requestGraphQL } from '~/gql/gql.server';

// Local
import type { Route } from './+types/dapp.$scope.$name.raw.plutus[.]json';

export async function loader({ context, params }: Route.LoaderArgs) {
  const id = `${params.dapp}/${params.scope}`;
  const result = await context.queryClient.fetchQuery({
    queryKey: dappQueryKeyGenerator(id),
    queryFn: requestGraphQL<{ dapp: Query['dapp']; }, QueryDappArgs>(
      SMALL_DAPP_QUERY,
      { scope: params.scope, name: params.name },
    ),
  });

  if (!result.dapp) {
    throw redirect('/');
  }

  let output = '{}';

  if (result.dapp.blueprintUrl) {
    let _finalUrl = result.dapp.blueprintUrl;

    if (_finalUrl.includes('github.com')) {
      _finalUrl = _finalUrl
        .replace('github.com', 'raw.githubusercontent.com')
        .replace('/blob', '');
    }

    // Improve it by storing on cache
    const blueprintResult = await fetch(_finalUrl);

    if (blueprintResult.ok) {
      output = await blueprintResult.text();
    }
  }

  return new Response(output, {
    status: 200,
    headers: {
      'Content-Type': 'application/json',
    },
  });
}
