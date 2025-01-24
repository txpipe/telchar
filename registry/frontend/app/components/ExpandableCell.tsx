import clsx from 'clsx';
import { forwardRef, useEffect, useImperativeHandle, useRef, useState, type ForwardedRef, type PropsWithChildren } from 'react';

import { ChevronDownBoldIcon } from '~/components/icons/chevron-down-bold';

interface Props {
  label: string;
}

interface Actions {
  expand(): void;
  close(): void;
}

function ExpandableCellComponent({ label, children }: PropsWithChildren<Props>, ref: ForwardedRef<Actions>) {
  const childrenRef = useRef<HTMLDivElement>(null);
  const [isExpanded, setIsExpanded] = useState(false);

  useImperativeHandle(ref, () => ({
    expand: () => setIsExpanded(true),
    close: () => setIsExpanded(false),
  }), []);

  useEffect(() => {
    if (childrenRef.current) {
      const elem = childrenRef.current;
      elem.style.maxHeight = isExpanded ? `${elem.scrollHeight}px` : '0px';
    }
  }, [isExpanded]);

  return (
    <div
      className="bg-white/5 border border-[#3E3E3E] px-3 py-4 cursor-pointer rounded-md"
      onClick={() => setIsExpanded(s => !s)}
      role="button"
    >
      <div className="flex flex-row gap-3 items-center text-white/95">
        <ChevronDownBoldIcon
          width="16"
          height="16"
          className={clsx('transition-transform duration-300', {
            'transform rotate-180': isExpanded,
          })}
        />
        <span>{label}</span>
      </div>
      <div
        className="overflow-hidden transition-all duration-300 ease-in-out"
        ref={childrenRef}
        aria-expanded={isExpanded}
        style={{ maxHeight: '0px' }}
      >
        {children}
      </div>
    </div>
  );
}

export const ExpandableCell = forwardRef(ExpandableCellComponent);
