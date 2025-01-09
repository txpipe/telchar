// Components
import { useState } from 'react';
import { Header } from '~/components/Header';
import { Hero } from '~/components/Hero';

// Local components
import { Catalogue } from './catalogue';

export function Home() {
  const [_search, setSearch] = useState('');

  return (
    <main className="container m-auto py-14">
      <Header />
      <Hero className="mt-14" onSearch={setSearch} />
      <Catalogue className="mt-20" />
    </main>
  );
}
