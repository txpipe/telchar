import { useState } from 'react';

// Components
import { Hero } from '~/components/Hero';

// Local components
import { Catalogue } from './catalogue';

export function Home({ initialDApps }: { initialDApps: DappConnection; }) {
  const [_search, setSearch] = useState('');

  return (
    <main className="mt-20">
      <Hero onSearch={setSearch} />
      <Catalogue
        className="mt-20"
        initialDApps={initialDApps}
      />
    </main>
  );
}
