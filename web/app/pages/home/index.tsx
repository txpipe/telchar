// Components
import { Header } from '~/components/Header';
import { Hero } from '~/components/Hero';
import { Card } from '~/components/ui/Card';

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

export function Home() {
  return (
    <main className="container m-auto py-14">
      <Header />
      <Hero className="mt-14" />
      <section className="mt-20">
        <div className="flex justify-between items-center">
          <h2 className="text-3xl font-semibold">Catalogue</h2>
          <div>
            Filter by: Newest
          </div>
        </div>
        <div className="grid grid-cols-2 gap-6 mt-11">
          {Array(6).fill(null).map((_, i) => (
            <PackageCard key={`card-${i}`} />
          ))}
        </div>
      </section>
    </main>
  );
}
