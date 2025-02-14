import clsx from 'clsx';
import type { ReactNode } from 'react';

interface TabProps {
  icon?: ReactNode;
  name: string;
  onClick?: () => void;
  active?: boolean;
}

export function TabName({ icon, name, onClick, active }: TabProps) {
  return (
    <button
      type="button"
      className={clsx(
        'flex items-center gap-2 pb-1.5 after:border-b-4 border-bottom-gradient border-transparent text-lg',
        active ? 'bg-gradient-to-r from-secondary-400 to-secondary-500 text-transparent bg-clip-text after:bg-gradient-to-r after:from-secondary-400 after:to-secondary-500 after:bg-clip-border' : 'text-white/60',
      )}
      onClick={onClick}
    >
      {icon}
      <span>{name}</span>
    </button>
  );
}
