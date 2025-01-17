import { useState } from 'react';

// Components
import { Header } from '~/components/Header';
import { Hero } from '~/components/Hero';
import { Footer } from '~/components/Footer';

// Local components
import { Catalogue } from './catalogue';

export function Home({ initialDApps }: { initialDApps: DappConnection; }) {
  const [_search, setSearch] = useState('');

  return (
    <>
      <Header />
      <main>
        <Hero className="mt-14" onSearch={setSearch} />
        <Catalogue
          className="mt-20"
          initialDApps={initialDApps}
        />
      </main>
      <Footer className="mt-20" />
    </>
  );
}
