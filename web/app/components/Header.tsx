import { Link, NavLink } from 'react-router';
import clsx from 'clsx';

interface Props {
  className?: string;
}

export function Header({ className }: Props) {
  return (
    <header className={clsx('flex justify-between items-center', className)}>
      <Link to="/" className="text-[32px] leading-relaxed tracking-[1.5%] font-semibold">Telchar</Link>

      <nav>
        <NavLink to="/docs">Documentation</NavLink>
      </nav>
    </header>
  );
}
