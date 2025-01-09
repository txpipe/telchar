import { Link, NavLink } from 'react-router';
import clsx from 'clsx';

interface Props {
  className?: string;
}

export function Header({ className }: Props) {
  return (
    <header className={clsx('flex justify-between items-center', className)}>
      <Link to="/" className="text-[34px] leading-relaxed tracking-[1.5%] font-semibold">Telchar</Link>

      <nav className="flex gap-8">
        <NavLink to="/docs">Documentation</NavLink>
        <NavLink to="/support">Support</NavLink>
      </nav>
    </header>
  );
}
