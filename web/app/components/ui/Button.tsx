import type { MouseEventHandler, PropsWithChildren } from 'react';
import { tv, type VariantProps } from 'tailwind-variants';

const button = tv({
  base: 'text-base font-semibold',
  variants: {
    spacing: {
      base: 'px-6 py-3.5 rounded-full',
      icon: 'p-1.5 rounded-lg',
    },
    color: {
      primary: 'bg-primary-400 text-white',
      'primary-gradient': 'bg-gradient-to-r from-primary-400 to-primary-500 text-white',
    },
    disabled: {
      true: 'bg-white/70 opacity-50 cursor-not-allowed',
    },
  },

  defaultVariants: {
    color: 'primary-gradient',
    spacing: 'base',
  },
});

type ButtonVariants = VariantProps<typeof button>;

interface Props extends ButtonVariants {
  type?: 'button' | 'submit' | 'reset';
  className?: string;
  onClick?: MouseEventHandler<HTMLButtonElement>;
}

export function Button({ type, children, onClick, ...buttonProps }: PropsWithChildren<Props>) {
  return (
    <button type={type} onClick={onClick} className={button(buttonProps)}>
      {children}
    </button>
  );
}
