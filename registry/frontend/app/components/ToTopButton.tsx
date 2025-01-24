import { useEffect, useState } from 'react';
import clsx from 'clsx';

// Components
import { Button } from '~/components/ui/Button';
import { ChevronDownBoldIcon } from '~/components/icons/chevron-down-bold';

export function ToTopButton() {
  const [isScrolled, setIsScrolled] = useState(false);

  useEffect(() => {
    const handleScroll = () => {
      setIsScrolled(window.scrollY > window.innerHeight / 2);
    };

    window.addEventListener('scroll', handleScroll);
    return () => window.removeEventListener('scroll', handleScroll);
  }, []);

  return (
    <Button
      spacing="icon"
      type="button"
      color="primary"
      className={clsx(
        'rounded-full p-4 fixed bottom-4 right-4 transition-opacity duration-300',
        { 'opacity-100': isScrolled, 'opacity-0': !isScrolled },
      )}
      onClick={() => {
        window.scrollTo({ top: 0, behavior: 'smooth' });
      }}
    >
      <ChevronDownBoldIcon className="transform rotate-180" />
    </Button>
  );
}
