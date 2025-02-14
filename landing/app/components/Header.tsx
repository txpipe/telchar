import { Link } from 'react-router';
import clsx from 'clsx';

// Components
import { MenuGridIcon } from '~/components/icons/menu-grid';

interface Props {
  className?: string;
}

export function Header({ className }: Props) {
  return (
    <header className={clsx('flex justify-between items-center', className)}>
      <div className="flex items-center gap-4 leading-relaxed tracking-[0.51px]">
        <Link to="/" className="text-[34px] font-semibold">Telchar</Link>
      </div>

      <nav className="flex items-center gap-8">
        <MenuGridIcon />
      </nav>
    </header>
  );
}
