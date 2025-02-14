// Pages
import { Home as HomePage } from '~/pages/home';

// Types
import type { Route } from './+types/_index';

export function meta({}: Route.MetaArgs) {
  return [
    { title: 'Telchar' },
    { name: 'description', content: 'Telchar description' },
  ];
}

export default function Home() {
  return <HomePage />;
}
