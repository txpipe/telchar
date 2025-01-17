import 'react-router';
import { createRequestHandler } from '@react-router/express';
import express from 'express';

import createQueryClient from './query';

declare module 'react-router' {
  interface AppLoadContext {
    queryClient: ReturnType<typeof createQueryClient>;
  }
}

export const app = express();
app.disable('x-powered-by');

app.use(
  createRequestHandler({
    // @ts-expect-error - virtual module provided by React Router at build time
    build: () => import('virtual:react-router/server-build'),
    getLoadContext() {
      return {
        queryClient: createQueryClient(),
      };
    },
  }),
);
