import { useState } from 'react';

// Components
import { Hero } from '~/components/Hero';
import { Header } from '~/components/Header';

// Local components
import { Catalogue } from './catalogue';

export function Home({ dapps }: { dapps: DappConnection; }) {
  const [_search, setSearch] = useState('');

  return (
    <>
      <Header appName="Registry" />
      <main className="mt-20">
        <Hero onSearch={setSearch} />
        <Catalogue
          className="mt-20"
          dapps={dapps}
        />
      </main>
    </>
  );
}
