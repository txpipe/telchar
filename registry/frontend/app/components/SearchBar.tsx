import clsx from 'clsx';
import { useRef, useState } from 'react';

// Components
import { SearchIcon } from '~/components/icons/search';
import { TimesIcon } from '~/components/icons/times';

interface Props {
  className?: string;
  onSearch?: (search: string) => void;
}

export function SearchBar({ className }: Props) {
  const [search, setSearch] = useState('');
  const searchRef = useRef<HTMLInputElement>(null);

  return (
    <search className={clsx('flex relative gap-8 w-full items-center', className)}>
      <SearchIcon className="absolute text-black left-6 pointer-events-none" width="26" height="26" />
      <input
        ref={searchRef}
        type="text"
        placeholder="Search dApps..."
        onChange={e => setSearch(e.target.value)}
        value={search}
        className="w-full rounded-full py-3.5 px-[70px] placeholder:text-black/50 placeholder:text-lg text-black"
      />
      <TimesIcon
        className={
          clsx(
            'absolute right-6 cursor-pointer text-black transition-opacity duration-200',
            search ? 'opacity-100' : 'opacity-0 pointer-events-none',
          )
        }
        width="26"
        height="26"
        onClick={e => {
          e.preventDefault();
          e.stopPropagation();
          setSearch('');
          searchRef.current?.focus();
        }}
      />
    </search>
  );
}
