import { redirect, type ShouldRevalidateFunctionArgs } from 'react-router';

// GQL
import { DAPP_QUERY, dappQueryKeyGenerator } from '~/gql/dapps/dapps.query';
import { requestGraphQL } from '~/gql/gql.server';

// Pages
import { DAppDetails as Page } from '~/pages/dapp/details';

// Local
import type { Route } from './+types/dapp.$scope.$name';

export function meta({ data }: Route.MetaArgs) {
  let title = 'Telchar';
  if (data.dapp?.name) {
    title = `${data.dapp.name} - Telchar`;
  }

  return [
    { title },
    { name: 'description', content: 'Telchar description' },
  ];
}

export async function loader({ context, params }: Route.LoaderArgs) {
  const id = `${params.dapp}/${params.scope}`;
  const result = await context.queryClient.fetchQuery({
    queryKey: dappQueryKeyGenerator(id),
    queryFn: requestGraphQL<{ dapp: Query['dapp']; }, QueryDappArgs>(
      DAPP_QUERY,
      { scope: params.scope, name: params.name },
    ),
  });

  if (!result.dapp) {
    throw redirect('/');
  }

  let readme: string | null = result.dapp.readme;

  if (!readme && result.dapp.repositoryUrl) {
    let repoPath = '';
    try {
      repoPath = (URL.parse(result.dapp.repositoryUrl)?.pathname ?? '').replace(/^\//, '');
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

export function shouldRevalidate(
  arg: ShouldRevalidateFunctionArgs,
) {
  // This will prevent to validate when activeTab changes
  return arg.currentParams.scope !== arg.nextParams.scope || arg.currentParams.name !== arg.nextParams.name;
}
