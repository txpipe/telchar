import { redirect } from 'react-router';

// GQL
import { DAPP_QUERY } from '~/gql/dapps/dapps.query';
import { requestGraphQL } from '~/gql/gql.server';

// Pages
import { DAppDetails as Page } from '~/pages/dapp/details';

// Local
import type { Route } from './+types/details';

export function meta({ data }: Route.MetaArgs) {
  let title = 'Telchar';
  if (data.dapp?.name) {
    title = `${data.dapp.name} - Telchar`;
  }

  return [
    { title },
    { name: 'description', content: 'Telchar descriptions' },
  ];
}

export async function loader({ context, params }: Route.LoaderArgs) {
  const id = `${params.dapp}/${params.scope}`;
  const result = await context.queryClient.fetchQuery({
    queryKey: ['dapp', id],
    queryFn: requestGraphQL<{ dapp: Query['dapp']; }, QueryDappArgs>(
      DAPP_QUERY,
      { id },
    ),
  });

  if (!result.dapp) {
    throw redirect('/');
  }

  let readme: string | null = null;

  if (result.dapp.repository) {
    let repoPath = '';
    try {
      repoPath = (URL.parse(result.dapp.repository)?.pathname ?? '').replace(/^\//, '');
    } catch {}

    // Improve it by storing on cache
    const readmeResult = await fetch(
      `https://api.github.com/repos/${repoPath}/readme`,
      {
        headers: {
          Accept: 'application/vnd.github.v3.raw',
        },
      },
    );

    if (readmeResult.ok) {
      readme = await readmeResult.text();
    }
  }

  return {
    dapp: result.dapp,
    readme,
  };
}

export default function DAppDetails({ loaderData }: Route.ComponentProps) {
  const { dapp, readme } = loaderData;

  return (
    <Page dapp={dapp} readme={readme} />
  );
}
