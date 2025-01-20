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
        'flex items-center gap-1.5 pb-1 border-b-4 transition-colors duration-500',
        active ? 'text-primary-400 border-b-primary-400' : 'text-white/60 border-b-transparent',
      )}
      onClick={onClick}
    >
      {icon}
      <span>{name}</span>
    </button>
  );
}
