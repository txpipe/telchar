import clsx from 'clsx';
import { useRef, useState } from 'react';

// Components
import { SearchIcon } from '~/components/icons/search';
import { TimesIcon } from '~/components/icons/times';
import { CopyIcon } from '~/components/icons/copy';
import { MacIcon } from '~/components/icons/mac';
import { ArrowRightIcon } from '~/components/icons/arrow-right';
import { Button } from '~/components/ui/Button';

interface Props {
  className?: string;
  onSearch?: (search: string) => void;
}

export function Hero({ className, onSearch }: Props) {
  const [search, setSearch] = useState('');
  const searchRef = useRef<HTMLInputElement>(null);

  return (
    <section className={clsx(className)}>
      <h1 className="text-4xl font-bold text-center">Registry of Plutus blueprints</h1>

      <search className="flex flex-row gap-8 mt-14 w-full max-w-[800px] mx-auto items-center">
        <div className="flex relative w-full items-center">
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
            className="absolute right-6 cursor-pointer text-black"
            width="26"
            height="26"
            onClick={e => {
              e.preventDefault();
              e.stopPropagation();
              setSearch('');
              searchRef.current?.focus();
            }}
          />
        </div>
        <Button
          type="button"
          onClick={() => {
            onSearch?.(search);
          }}
        >
          Search
        </Button>
      </search>

      {/* <div className="relative flex flex-col items-center mt-20 p-8 before:bg-gradient-to-b before:from-[#434343]/[0.86] before:to-transparent before:rounded-t-xl before:absolute before:top-0 before:bottom-0 before:left-0 before:right-0 before:-z-10 z-0 before:pointer-events-none before:opacity-[0.41]"> */}
      <div className="flex flex-col items-center mt-20 p-8 bg-gradient-to-b from-[#434343]/[0.35] to-transparent rounded-t-xl backdrop-blur-[14.605px]">
        <h2 className="text-2xl font-semibold text-primary-400">Getting started</h2>
        <p className="mt-6 max-w-[800px] text-center">
          Lorem ipsum dolor sit amet consectetur. Nec diam lacus auctor pretium lorem in montes suspendisse.
          Nunc magna habitant cras curabitur mattis. Tristique felis penatibus turpis non mollis sed. Ultricies augue.
        </p>
        <div className="flex gap-6 mt-8 items-center">
          <p className="px-8 py-3.5 border border-white/30 rounded-full flex items-center gap-3 font-roboto text-lg">
            brew install gh
            <CopyIcon />
          </p>
          <span className="text-white/50">or</span>
          <p className="px-8 py-3.5 border border-white/30 rounded-full flex items-center gap-3 text-lg">
            Download for Mac
            <MacIcon />
          </p>
          <p className="flex items-center gap-3">
            View instalation instructions
            <ArrowRightIcon className="inline-block" />
          </p>
        </div>
      </div>
    </section>
  );
}
