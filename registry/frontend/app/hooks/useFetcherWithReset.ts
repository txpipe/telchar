import { useEffect, useState } from 'react';
import { useFetcher } from 'react-router';

export type FetcherWithComponentsReset<T> = ReturnType<typeof useFetcher<T>> & {
  reset: () => void;
};

export function useFetcherWithReset<T>(props: { key?: string; }): FetcherWithComponentsReset<T> {
  const fetcher = useFetcher<T>(props);
  const [data, setData] = useState(fetcher.data);

  useEffect(() => {
    if (fetcher.state === 'idle') {
      setData(fetcher.data);
    }
  }, [fetcher.state, fetcher.data]);

  return {
    ...fetcher,
    data,
    reset: () => setData(undefined),
  };
}
