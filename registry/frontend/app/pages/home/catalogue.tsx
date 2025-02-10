import { Link, useSearchParams } from 'react-router';

// Components
import { Card } from '~/components/ui/Card';
import { Button } from '~/components/ui/Button';
import { ChevronLeftIcon } from '~/components/icons/chevron-left';
import { ChevronRightIcon } from '~/components/icons/chevron-right';

// GQL
import { DAPPS_DEFAULT_PAGINATION } from '~/gql/dapps/dapps.query';

interface CatalogueProps {
  className?: string;
  dapps: DappConnection;
}

function PackageCard({ dapp }: { dapp: Dapp; }) {
  return (
    <Link to={`/dapp/${dapp.scope}/${dapp.name}`}>
      <Card>
        <h3 className="text-lg font-semibold">{dapp.name}</h3>
        <div>
          <span className="text-primary-400">@{dapp.scope ?? ''}</span>
          <span> • </span>
          <span>v{dapp.version}</span>
        </div>
      </Card>
    </Link>
  );
}

export function Catalogue({ className, dapps }: CatalogueProps) {
  const [searchParams, setSearchParams] = useSearchParams({ page: DAPPS_DEFAULT_PAGINATION.page.toString() });

  const page = parseInt(searchParams.get('page') ?? '0', 10) || DAPPS_DEFAULT_PAGINATION.page;

  const startCursor = +(dapps.pageInfo.startCursor ?? 0) + 1;
  const endCursor = +(dapps.pageInfo.endCursor ?? 0) + 1;
  const totalNodes = dapps.metadata?.totalNodes ?? 0;
  return (
    <section className={className}>
      <div className="flex justify-between items-center">
        <h2 className="text-3xl font-semibold">dApps</h2>
        <div>
          Filter by: Newest
        </div>
      </div>
      <div className="grid grid-cols-3 gap-6 mt-8">
        {dapps.nodes.map(dapp => (
          <PackageCard key={`dapp-${dapp.id}`} dapp={dapp} />
        ))}
      </div>
      <div className="mt-8 flex items-center justify-center gap-6">
        <span className="text-white/50">
          Displaying {startCursor}-{endCursor} of {totalNodes}
        </span>
        <div className="flex gap-3">
          <Button
            type="button"
            color="primary"
            spacing="icon"
            disabled={!dapps.pageInfo.hasPreviousPage}
            onClick={() => {
              searchParams.set('page', `${Math.max(1, page - 1)}`);
              setSearchParams(searchParams, { preventScrollReset: true });
            }}
          >
            <ChevronLeftIcon />
          </Button>
          <Button
            type="button"
            color="primary"
            spacing="icon"
            disabled={!dapps.pageInfo.hasNextPage}
            onClick={() => {
              searchParams.set('page', `${page + 1}`);
              setSearchParams(searchParams, { preventScrollReset: true });
            }}
          >
            <ChevronRightIcon />
          </Button>
        </div>
      </div>
    </section>
  );
}
