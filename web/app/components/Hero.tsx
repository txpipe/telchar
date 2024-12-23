import clsx from 'clsx';

// Components
import { Button } from '~/components/ui/Button';

interface Props {
  className?: string;
}

export function Hero({ className }: Props) {
  return (
    <section className={clsx(className)}>
      <h1 className="text-5xl font-bold">Registry</h1>

      <div className="flex flex-col items-center mt-14">
        <h2 className="text-2xl font-semibold text-primary-400">Getting started</h2>
        <p className="mt-6 max-w-[800px] text-center">
          Lorem ipsum dolor sit amet consectetur. Nec diam lacus auctor pretium lorem in montes suspendisse.
          Nunc magna habitant cras curabitur mattis. Tristique felis penatibus turpis non mollis sed. Ultricies augue.
        </p>
        <p className="mt-8 px-8 py-3.5 border border-white/30 rounded-md">
          npm i perfectionist-dfd
        </p>
        <search className="flex flex-row gap-8 mt-22 w-full max-w-[800px]">
          <input
            type="text"
            placeholder="Search packages...."
            className="w-full rounded-md py-3.5 px-8 placeholder:text-black/50 placeholder:text-lg"
          />
          <Button>Search</Button>
        </search>
      </div>
    </section>
  );
}
