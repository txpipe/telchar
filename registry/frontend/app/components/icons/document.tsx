import type { SVGProps } from 'react';

interface DocumentIconProps extends SVGProps<SVGSVGElement> {
  gradient?: 'secondary';
}

export function DocumentIcon(
  { gradient, ...props }: DocumentIconProps,
) {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24" {...props}>
      <path
        stroke={gradient ? `url(#${gradient})` : 'currentColor'}
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M22 10.442v4.666c0 4.667-2 6.534-7 6.534H9c-5 0-7-1.867-7-6.534v-5.6c0-4.666 2-6.533 7-6.533h5m8 7.467h-4c-3 0-4-.934-4-3.734V2.975m8 7.467-8-7.467"
      />
      <linearGradient id="secondary" x1="15.297" x2="22.886" y1="2.975" y2="4.022" gradientUnits="userSpaceOnUse">
        <stop stopColor="#3BE0FE" />
        <stop offset="1" stopColor="#0094AE" />
      </linearGradient>
    </svg>
  );
}
