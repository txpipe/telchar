import { Link, NavLink } from 'react-router';
import clsx from 'clsx';

// Components
// import { MenuGridIcon } from '~/components/icons/menu-grid';

interface Props {
  appName?: string;
  appLink?: string;
  className?: string;
}

export function Header({ className, appName, appLink }: Props) {
  return (
    <header className={clsx('flex justify-between items-center', className)}>
      <div className="flex items-center gap-4 leading-relaxed tracking-[1.5%]">
        <Link to="/" className="text-[34px] font-semibold">Telchar</Link>
        {!!appName && (
          <>
            <div className="bg-white/50 w-1 h-1 rounded-full" />
            <Link to={appLink ?? '/'} className="text-white/50 text-2xl">{appName}</Link>
          </>
        )}
      </div>

      <nav className="flex items-center gap-8">
        <NavLink to={import.meta.env.VITE_DOCS_URL} target="_blank" rel="noreferrer">Docs</NavLink>
        {/* <NavLink to="/support">Support</NavLink> */}
        {/* <MenuGridIcon /> */}
      </nav>
    </header>
  );
}
