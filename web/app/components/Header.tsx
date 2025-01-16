import { Link, NavLink } from 'react-router';
import clsx from 'clsx';

// Components
import { SearchBar } from '~/components/SearchBar';

interface Props {
  className?: string;
  withoutNav?: boolean;
  withSearch?: boolean;
  onSearch?: (search: string) => void;
}

export function Header({ className, withoutNav, withSearch, onSearch }: Props) {
  return (
    <header className={clsx('flex justify-between items-center', className)}>
      <Link to="/" className="text-[34px] leading-relaxed tracking-[1.5%] font-semibold">Telchar</Link>

      {!withoutNav && (
        <nav className="flex gap-8">
          <NavLink to="/docs">Documentation</NavLink>
          <NavLink to="/support">Support</NavLink>
        </nav>
      )}

      {withSearch && (
        <SearchBar onSearch={onSearch} className="max-w-[750px]" />
      )}
    </header>
  );
}
