// Components
import { Card } from '~/components/ui/Card';
import { Button } from '~/components/ui/Button';
import { ChevronLeftIcon } from '~/components/icons/chevron-left';
import { ChevronRightIcon } from '~/components/icons/chevron-right';

interface CatalogueProps {
  className?: string;
}

function PackageCard() {
  return (
    <Card>
      <div className="flex justify-between items-center">
        <h3 className="text-lg font-semibold">Package name</h3>
        <p className="text-xl">1,700</p>
      </div>
      <div>
        <span>v4.1.11</span>
        <span> • </span>
        <span className="text-primary-400">@asteria</span>
      </div>
    </Card>
  );
}

export function Catalogue({ className }: CatalogueProps) {
  return (
    <section className={className}>
      <div className="flex justify-between items-center">
        <h2 className="text-3xl font-semibold">dApps</h2>
        <div>
          Filter by: Newest
        </div>
      </div>
      <div className="grid grid-cols-3 gap-6 mt-8">
        {Array(6).fill(null).map((_, i) => (
          <PackageCard key={`card-${i}`} />
        ))}
      </div>
      <div className="mt-8 flex items-center justify-center gap-6">
        <span className="text-white/50">Displaying 1-15 of 150</span>
        <div className="flex gap-3">
          <Button color="primary" spacing="icon" disabled>
            <ChevronLeftIcon />
          </Button>
          <Button color="primary" spacing="icon">
            <ChevronRightIcon />
          </Button>
        </div>
      </div>
    </section>
  );
}
