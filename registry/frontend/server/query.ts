import { QueryClient } from '@tanstack/query-core';

export default () => new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: Number.POSITIVE_INFINITY,
    },
  },
});
