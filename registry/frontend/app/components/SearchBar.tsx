import clsx from 'clsx';
import { useEffect, useRef, useState } from 'react';
import { Link } from 'react-router';
import { twMerge } from 'tailwind-merge';

// Components
import { SearchIcon } from '~/components/icons/search';
import { TimesIcon } from '~/components/icons/times';

// Hooks
import { useDebounce } from '~/hooks/useDebounce';
import { useFetcherWithReset } from '~/hooks/useFetcherWithReset';

interface Props {
  className?: string;
  onSearch?: (search: string) => void;
  dark?: boolean;
}

function SearchResult({ dapp, onResultClick }: { dapp: Dapp; onResultClick?: () => void; }) {
  return (
    <Link
      to={`/dapp/${dapp.scope}/${dapp.name}`}
      className="text-black/90 hover:bg-primary-400/10 rounded-xl py-1.5 px-4 block"
      onClick={onResultClick}
    >
      <div className="text-lg font-semibold">{dapp.name}</div>
      <div>
        <span className="text-primary-400">@{dapp.scope}</span>
        <span> • </span>
        <span>v{dapp.version}</span>
      </div>
    </Link>
  );
}

export function SearchBar({ className, dark }: Props) {
  const [search, setSearch] = useState('');
  const debouncedSearch = useDebounce(search, 300);
  const searchRef = useRef<HTMLInputElement>(null);
  const fetcher = useFetcherWithReset<{ dapps: DappConnection; }>({ key: 'search-fetcher' });

  useEffect(() => {
    if (debouncedSearch) {
      fetcher.load(`/api/search?q=${encodeURIComponent(debouncedSearch)}`);
    } else {
      fetcher.reset();
    }
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [debouncedSearch]);

  const resetInput = () => {
    setSearch('');
    searchRef.current?.focus();
    fetcher.reset();
  };

  const { nodes } = fetcher.data?.dapps || { nodes: [] };

  return (
    <div className="flex items-center gap-14">
      <search className={twMerge('flex relative gap-8 w-full items-center text-black', className, dark && 'text-white/90')}>
        <SearchIcon className="absolute left-6 pointer-events-none" width="26" height="26" />
        <input
          ref={searchRef}
          type="text"
          placeholder="Search dApps..."
          onChange={e => setSearch(e.target.value)}
          value={search}
          className={twMerge('w-full rounded-full py-3.5 px-[68px] placeholder:text-black/50 placeholder:text-lg', dark && 'border border-white/50 bg-white/[0.02] placeholder:text-white/50')}
        />
        <TimesIcon
          className={
            clsx(
              'absolute right-6 cursor-pointer transition-opacity duration-200',
              search ? 'opacity-100' : 'opacity-0 pointer-events-none',
            )
          }
          width="26"
          height="26"
          onClick={e => {
            e.preventDefault();
            e.stopPropagation();
            resetInput();
          }}
        />
        {nodes.length > 0 && (
          <div className="absolute left-0 right-0 -bottom-4 bg-white rounded-xl px-8 py-6 translate-y-full z-10">
            <span className="text-black/50">Top results</span>
            <ul className="max-h-64 overflow-y-auto mt-4">
              {nodes.map(dapp => (
                <li key={dapp.id}>
                  <SearchResult dapp={dapp} onResultClick={resetInput} />
                </li>
              ))}
            </ul>
          </div>
        )}
      </search>
    </div>
  );
}
