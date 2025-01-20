import type { PropsWithChildren } from 'react';
import { tv, type VariantProps } from 'tailwind-variants';

const card = tv({
  base: 'z-0 relative rounded-[10px] card py-6 px-8',
  variants: {
    color: {
      primary: 'bg-gradient-to-b from-[#FF02E6]/[0.12] to-white/0',
    },
    borderColor: {
      white: 'after:bg-gradient-to-b after:from-white/60 after:via-transparent after:to-white/60 after:bg-origin-border',
    },
  },

  defaultVariants: {
    color: 'primary',
    borderColor: 'white',
  },
});

type CardVariants = VariantProps<typeof card>;

interface Props extends CardVariants {
  className?: string;
}

export function Card({ children, ...cardProps }: PropsWithChildren<Props>) {
  return (
    <div className={card(cardProps)}>
      {children}
    </div>
  );
}
