import { Home as HomePage } from '~/pages/home';
import type { Route } from './+types/home';

export function meta({}: Route.MetaArgs) {
  return [
    { title: 'Telchar' },
    { name: 'description', content: 'Telchar descriptions' },
  ];
}

export default function Home() {
  return <HomePage />;
}
