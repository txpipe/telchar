import type { SVGProps } from 'react';

interface DeploymentIconProps extends SVGProps<SVGSVGElement> {
  gradient?: 'secondary';
}

export function DeploymentIcon(
  { gradient, ...props }: DeploymentIconProps,
) {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24" {...props}>
      <path
        stroke={gradient ? `url(#${gradient}01)` : 'currentColor'}
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M22 9.458v-4.04c0-1.59-.64-2.23-2.23-2.23h-4.04c-1.59 0-2.23.64-2.23 2.23v4.04c0 1.59.64 2.23 2.23 2.23h4.04c1.59 0 2.23-.64 2.23-2.23Z"
      />
      <path
        stroke={gradient ? `url(#${gradient}02)` : 'currentColor'}
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M10.5 9.707v-4.54c0-1.41-.64-1.98-2.23-1.98H4.23c-1.59 0-2.23.57-2.23 1.98v4.53c0 1.42.64 1.98 2.23 1.98h4.04c1.59.01 2.23-.56 2.23-1.97Z"
      />
      <path
        stroke={gradient ? `url(#${gradient}03)` : 'currentColor'}
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M10.5 20.955v-4.04c0-1.59-.64-2.23-2.23-2.23H4.23c-1.59 0-2.23.64-2.23 2.23v4.04c0 1.59.64 2.23 2.23 2.23h4.04c1.59 0 2.23-.64 2.23-2.23Z"
      />
      <path
        stroke={gradient ? `url(#${gradient}04)` : 'currentColor'}
        strokeLinecap="round"
        d="M14.5 18.683h6"
      />
      <path
        stroke={gradient ? `url(#${gradient}05)` : 'currentColor'}
        strokeLinecap="round"
        d="M17.5 21.683v-6"
      />
      <defs>
        <linearGradient id="secondary01" x1="14.878" x2="22.96" y1="3.188" y2="4.228" gradientUnits="userSpaceOnUse">
          <stop stopColor="#3BE0FE" />
          <stop offset="1" stopColor="#0094AE" />
        </linearGradient>
        <linearGradient id="secondary02" x1="3.378" x2="11.46" y1="3.188" y2="4.229" gradientUnits="userSpaceOnUse">
          <stop stopColor="#3BE0FE" />
          <stop offset="1" stopColor="#0094AE" />
        </linearGradient>
        <linearGradient id="secondary03" x1="3.378" x2="11.46" y1="14.685" y2="15.726" gradientUnits="userSpaceOnUse">
          <stop stopColor="#3BE0FE" />
          <stop offset="1" stopColor="#0094AE" />
        </linearGradient>
        <linearGradient id="secondary04" gradientUnits="userSpaceOnUse">
          <stop stopColor="#3BE0FE" />
          <stop offset="1" stopColor="#0094AE" />
        </linearGradient>
        <linearGradient id="secondary05" gradientUnits="userSpaceOnUse">
          <stop stopColor="#3BE0FE" />
          <stop offset="1" stopColor="#0094AE" />
        </linearGradient>
      </defs>
    </svg>
  );
}
