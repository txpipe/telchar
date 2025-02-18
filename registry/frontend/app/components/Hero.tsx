import clsx from 'clsx';

// Components
import { CopyIcon } from '~/components/icons/copy';
import { ArrowRightIcon } from '~/components/icons/arrow-right';
import { SearchBar } from '~/components/SearchBar';

interface Props {
  className?: string;
  onSearch?: (search: string) => void;
}

export function Hero({ className, onSearch }: Props) {
  return (
    <section className={clsx(className)}>
      <h1 className="text-4xl font-bold text-center">Registry of Plutus blueprints</h1>

      <SearchBar className="mt-14 mx-auto  max-w-[836px]" onSearch={onSearch} />

      <div className="flex flex-col items-center mt-20 p-8 bg-[linear-gradient(136.71deg,var(--tw-gradient-stops))] from-[#434343]/[0.35] to-transparent rounded-t-xl backdrop-blur-[7.30274px]">
        <h2 className="text-2xl font-semibold text-primary-400">Getting started</h2>
        <p className="mt-6 max-w-[800px] text-center">
          Download and run Telchar effortlessly with our standalone binaries or installers for various systems and
          architectures. Choose the option that best fits your needs and get started in seconds.
        </p>
        <div className="flex gap-6 mt-8 items-center">
          <p className="px-8 py-3.5 border border-white/30 rounded-full flex items-center gap-3 font-roboto text-lg">
            brew install txpipe/tap/telchar
            <button type="button" onClick={() => navigator.clipboard.writeText('brew install txpipe/tap/telchar')}>
              <CopyIcon />
            </button>
          </p>
          <span className="text-white/50">or</span>
          <a
            href={`${import.meta.env.VITE_DOCS_URL}/cli`}
            className="flex items-center gap-3"
            target="_blank"
            rel="noreferrer"
          >
            View installation instructions
            <ArrowRightIcon className="inline-block" />
          </a>
        </div>
      </div>
    </section>
  );
}
