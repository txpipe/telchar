// Components
import { Card } from '~/components/ui/Card';
import { Button } from '~/components/ui/Button';
import { ChevronLeftIcon } from '~/components/icons/chevron-left';
import { ChevronRightIcon } from '~/components/icons/chevron-right';

interface CatalogueProps {
  className?: string;
  initialDApps: Dapp[];
}

function PackageCard({ dapp }: { dapp: Dapp; }) {
  return (
    <Card>
      <h3 className="text-lg font-semibold">{dapp.name}</h3>
      <div>
        <span className="text-primary-400">@{dapp.team?.name ?? ''}</span>
        <span> • </span>
        <span>1 year ago</span>
      </div>
    </Card>
  );
}

export function Catalogue({ className, initialDApps }: CatalogueProps) {
  return (
    <section className={className}>
      <div className="flex justify-between items-center">
        <h2 className="text-3xl font-semibold">dApps</h2>
        <div>
          Filter by: Newest
        </div>
      </div>
      <div className="grid grid-cols-3 gap-6 mt-8">
        {initialDApps.map(dapp => (
          <PackageCard key={`dapp-${dapp.id}`} dapp={dapp} />
        ))}
      </div>
      <div className="mt-8 flex items-center justify-center gap-6">
        <span className="text-white/50">Displaying 1-{initialDApps.length} of {initialDApps.length}</span>
        <div className="flex gap-3">
          <Button color="primary" spacing="icon" disabled>
            <ChevronLeftIcon />
          </Button>
          <Button color="primary" spacing="icon" disabled>
            <ChevronRightIcon />
          </Button>
        </div>
      </div>
    </section>
  );
}
