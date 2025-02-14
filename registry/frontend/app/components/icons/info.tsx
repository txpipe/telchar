import type { SVGProps } from 'react';

interface InfoIconProps extends SVGProps<SVGSVGElement> {
  gradient?: 'secondary';
}

export function InfoIcon(
  { gradient, ...props }: InfoIconProps,
) {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24" {...props}>
      <path
        stroke={gradient ? `url(#${gradient}01)` : 'currentColor'}
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M12 2.97c-5.5 0-10 4.2-10 9.334 0 5.133 4.5 9.333 10 9.333s10-4.2 10-9.333-4.5-9.333-10-9.333Z"
      />
      <path
        stroke={gradient ? `url(#${gradient}02)` : 'currentColor'}
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M12 16.037v-4.666"
      />
      <path
        stroke={gradient ? `url(#${gradient}03)` : 'currentColor'}
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M12.006 8.575h-.01"
      />
      <defs>
        <linearGradient id="secondary01" x1="18.759" x2="-.214" y1="21.637" y2="19.02" gradientUnits="userSpaceOnUse">
          <stop stopColor="#3BE0FE" />
          <stop offset="1" stopColor="#0094AE" />
        </linearGradient>
        <linearGradient id="secondary02" gradientUnits="userSpaceOnUse">
          <stop stopColor="#3BE0FE" />
          <stop offset="1" stopColor="#0094AE" />
        </linearGradient>
        <linearGradient id="secondary03" gradientUnits="userSpaceOnUse">
          <stop stopColor="#3BE0FE" />
          <stop offset="1" stopColor="#0094AE" />
        </linearGradient>
      </defs>
    </svg>
  );
}
