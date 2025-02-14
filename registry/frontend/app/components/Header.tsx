import { Link, NavLink } from 'react-router';
import clsx from 'clsx';

// Components
// import { MenuGridIcon } from '~/components/icons/menu-grid';

interface Props {
  appName?: string;
  appLink?: string;
  className?: string;
  centerNode?: React.ReactNode;
}

export function Header({ className, appName, appLink, centerNode }: Props) {
  return (
    <header className={clsx('header', className)}>
      <div className="flex items-center gap-4 leading-relaxed tracking-[0.51px]" style={{ gridArea: 'logo' }}>
        <Link to="/" className="text-[34px] font-semibold">Telchar</Link>
        {!!appName && (
          <>
            <div className="bg-white/50 w-1 h-1 rounded-full" />
            <Link to={appLink ?? '/'} className="text-white/50 text-2xl">{appName}</Link>
          </>
        )}
      </div>
      <div style={{ gridArea: 'center' }}>
        {centerNode}
      </div>

      <nav className="flex items-center justify-end gap-8" style={{ gridArea: 'nav' }}>
        <NavLink to={import.meta.env.VITE_DOCS_URL} target="_blank" rel="noreferrer">Docs</NavLink>
        {/* <NavLink to="/support">Support</NavLink> */}
        {/* <MenuGridIcon /> */}
      </nav>
    </header>
  );
}
