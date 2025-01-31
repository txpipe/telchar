import type { MouseEventHandler, PropsWithChildren } from 'react';
import { tv, type VariantProps } from 'tailwind-variants';

const button = tv({
  base: 'text-base flex flex-row gap-3',
  variants: {
    spacing: {
      compact: 'py-2 px-6 rounded-full',
      base: 'px-6 py-3.5 rounded-full',
      icon: 'p-1.5 rounded-lg',
    },
    text: {
      base: 'text-base',
      large: 'text-lg',
    },
    weight: {
      semibold: 'font-semibold',
      normal: 'font-normal',
    },
    outlined: {
      true: 'bg-transparent',
    },
    color: {
      primary: 'bg-primary-400 text-white',
      'primary-gradient': 'bg-gradient-to-r from-primary-400 to-primary-500 text-white',
      white: 'bg-white text-primary-400',
    },
    disabled: {
      true: 'bg-white/70 opacity-50 cursor-not-allowed',
    },
  },

  compoundVariants: [
    {
      outlined: true,
      color: 'primary',
      class: 'border border-primary-400/30 text-primary-400',
    },
    {
      outlined: true,
      color: 'white',
      class: 'bg-transparent border border-white/30 text-white',
    },
    {
      outlined: true,
      color: 'primary-gradient',
      class: 'after:rounded-full bg-clip-text text-transparent btn-outline-primary-gradient after:bg-gradient-to-r after:from-primary-400 after:to-primary-500 after:bg-origin-border',
    },
  ],

  defaultVariants: {
    color: 'primary-gradient',
    spacing: 'base',
    text: 'base',
    weight: 'semibold',
    solid: true,
  },
});

type ButtonVariants = VariantProps<typeof button>;

interface Props extends ButtonVariants {
  type?: 'button' | 'submit' | 'reset';
  className?: string;
  onClick?: MouseEventHandler<HTMLButtonElement>;
  leftIcon?: React.ReactNode;
  rightIcon?: React.ReactNode;
}

export function Button({ type, children, onClick, ...buttonProps }: PropsWithChildren<Props>) {
  return (
    <button type={type} onClick={!buttonProps.disabled ? onClick : undefined} className={button(buttonProps)}>
      {children}
    </button>
  );
}
