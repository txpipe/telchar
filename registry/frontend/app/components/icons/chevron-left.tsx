import type { SVGProps } from 'react';

export function ChevronLeftIcon(props: SVGProps<SVGSVGElement>) {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24" {...props}>
      <path
        stroke="currentColor"
        strokeLinecap="round"
        strokeWidth="1.5"
        d="m14.387 16.8-4.8-4.8 4.8-4.8"
      />
    </svg>
  );
}
