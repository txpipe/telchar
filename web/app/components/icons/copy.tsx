import type { SVGProps } from 'react';

export function CopyIcon(props: SVGProps<SVGSVGElement>) {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24" {...props}>
      <path
        fill="currentColor"
        d="M9 3.75A5.75 5.75 0 0 0 3.25 9.5v7.107a.75.75 0 1 0 1.5 0V9.5A4.25 4.25 0 0 1 9 5.25h7.013a.75.75 0 1 0 0-1.5H9Z"
      />
      <path
        fill="currentColor"
        fillRule="evenodd"
        d="M18.403 7.293a44.4 44.4 0 0 0-9.806 0 2.01 2.01 0 0 0-1.774 1.76 42.6 42.6 0 0 0 0 9.894 2.01 2.01 0 0 0 1.774 1.76c3.24.362 6.565.362 9.806 0a2.01 2.01 0 0 0 1.774-1.76 42.598 42.598 0 0 0 0-9.894 2.01 2.01 0 0 0-1.774-1.76Zm-9.64 1.491c3.13-.35 6.343-.35 9.473 0a.51.51 0 0 1 .45.444c.371 3.17.371 6.373 0 9.544a.51.51 0 0 1-.45.444c-3.13.35-6.342.35-9.472 0a.51.51 0 0 1-.45-.444 41 41 0 0 1 0-9.544.51.51 0 0 1 .45-.444Z"
        clipRule="evenodd"
      />
    </svg>
  );
}
