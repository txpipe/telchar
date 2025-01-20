import clsx from 'clsx';
import dayjs from 'dayjs';
import { CopyIcon } from '~/components/icons/copy';
import { GitIcon } from '~/components/icons/git';

interface Props {
  className?: string;
  dapp: Dapp;
}

export function Info({ dapp, className }: Props) {
  return (
    <div className={clsx('grid grid-cols-1 gap-8 w-full', className)}>
      <p className="text-2xl font-semibold">{dapp.name}</p>

      <div>
        <p className="text-white/50">Published by</p>
        <p className="mt-4 text-primary-400">@{dapp.scope?.name ?? ''}</p>
      </div>

      <div>
        <p className="text-white/50">Repository</p>
        <a href={dapp.repository} className="w-fit mt-4 text-white flex items-center gap-2.5">
          <GitIcon width="15" height="15" />
          {dapp.repository}
        </a>
      </div>

      <div>
        <p className="text-white/50">Codegen</p>
        <p className="w-fit mt-4 px-8 py-4 flex items-center gap-3 font-roboto bg-gradient-to-b from-[#434343]/[0.35] to-transparent rounded-lg backdrop-blur-[14.605px]">
          <span>telchar codegen {dapp.scope?.name ?? ''}/{dapp.name}</span>
          <CopyIcon />
        </p>
      </div>

      <div className="flex flex-col gap-2">
        <div>
          <span className="text-white/30">Plutus version</span>
          <span className="text-white/50 ml-4">v3</span>
        </div>
        <div>
          <span className="text-white/30">Compiler</span>
          <span className="text-white/50 ml-4">Aiken v1.1.3+3d77b5c</span>
        </div>
        <div>
          <span className="text-white/30">License</span>
          <span className="text-white/50 ml-4">Apache-2.0</span>
        </div>
        <div>
          <span className="text-white/30">Publication date</span>
          <span className="text-white/50 ml-4">Published {dapp.publishedDate ? dayjs(new Date(dapp.publishedDate * 1000)).fromNow() : ''}</span>
        </div>
      </div>

    </div>
  );
}
