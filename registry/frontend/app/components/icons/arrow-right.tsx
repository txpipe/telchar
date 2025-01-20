import type { SVGProps } from 'react';

export function ArrowRightIcon(props: SVGProps<SVGSVGElement>) {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24" {...props}>
      <path
        fill="currentColor"
        d="m20 12.5.354-.354.353.354-.353.354-.354-.353ZM5 13a.5.5 0 0 1 0-1v1Zm9.354-6.854 6 6-.708.708-6-6 .708-.708Zm6 6.708-6 6-.708-.707 6-6 .708.707Zm-.354.147H5v-1h15v1Z"
      />
    </svg>
  );
}
