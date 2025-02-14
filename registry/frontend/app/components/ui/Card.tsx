import type { PropsWithChildren } from 'react';
import { tv, type VariantProps } from 'tailwind-variants';

const card = tv({
  base: 'z-0 relative rounded-[10px] py-6 px-8',
  variants: {
    color: {
      'primary-gradient': 'bg-gradient-to-b from-[#FF02E6]/[0.12] to-white/0',
      white: 'bg-white/[0.02]',
    },
    borderColor: {
      white: 'border border-white/10',
    },
  },

  defaultVariants: {
    color: 'white',
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
