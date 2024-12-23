import type { PropsWithChildren } from 'react';
import { tv, type VariantProps } from 'tailwind-variants';

const button = tv({
  base: 'px-6 py-3.5 rounded-full',
  variants: {
    color: {
      primary: 'bg-primary-400 text-white',
      'primary-gradient': 'bg-gradient-to-r from-primary-400 to-primary-500 text-white',
    },
  },

  defaultVariants: {
    color: 'primary-gradient',
  },
});

type ButtonVariants = VariantProps<typeof button>;

interface Props extends ButtonVariants {
  type?: 'button' | 'submit' | 'reset';
  className?: string;

}

export function Button({ type, children, ...buttonProps }: PropsWithChildren<Props>) {
  return (
    <button type={type} className={button(buttonProps)}>
      {children}
    </button>
  );
}
