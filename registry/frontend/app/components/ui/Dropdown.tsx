import { Menu, MenuButton, MenuItem, MenuItems } from '@headlessui/react';
import clsx from 'clsx';
import { useEffect, useState } from 'react';

// Components
import { ChevronDownIcon } from '~/components/icons/chevron-down';

interface Props {
  label?: string;
  value: string;
  showValue?: boolean;
  options: { label: string; value: string; }[];
  onOptionSelected?: (value: string) => void;
}

export function Dropdown({ label, value, showValue, options, onOptionSelected }: Props) {
  const [activeOption, setActiveOption] = useState(options.find(option => option.value === value));

  useEffect(() => {
    if (value !== activeOption?.value) {
      setActiveOption(options.find(option => option.value === value));
    }

  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [value]);

  return (
    <Menu>
      <MenuButton className="flex flex-row gap-2 py-3 px-4 rounded-full items-center text-white text-sm border border-white/50">
        {!!label && (
          <span className="opacity-50">{label}</span>
        )}
        {(showValue && activeOption) && (
          <span>{activeOption?.label}</span>
        )}
        <ChevronDownIcon width="14" height="14" />
      </MenuButton>
      <MenuItems anchor="bottom end" className="bg-white rounded-xl p-4 flex flex-col gap-2 text-black/90 min-w-64 mt-0.5">
        {options.map(option => (
          <MenuItem key={option.value}>
            <button
              type="button"
              className={clsx(
                'text-left py-1.5 px-2 rounded-[10px] transition-all hover:bg-primary-400/10 hover:font-semibold',
                { 'text-primary-400 font-semibold': option.value === activeOption?.value },
              )}
              onClick={() => {
                setActiveOption(option);
                onOptionSelected?.(option.value);
              }}
            >
              {option.label}
            </button>
          </MenuItem>
        ))}
      </MenuItems>
    </Menu>
  );
}
