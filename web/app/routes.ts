import { type RouteConfig, index, prefix, route } from '@react-router/dev/routes';

export default [
  index('routes/home.tsx'),
  ...prefix('dapp', [
    route(':scope/:dapp', 'routes/dapp/details.tsx'),
  ]),
] satisfies RouteConfig;
