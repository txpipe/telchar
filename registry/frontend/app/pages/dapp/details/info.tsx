import clsx from 'clsx';
import dayjs from 'dayjs';

// Components
import { CopyIcon } from '~/components/icons/copy';
import { GitIcon } from '~/components/icons/git';

interface Props {
  className?: string;
  dapp: Dapp;
}

function CommonDetails({ label, value }: { label: string; value: string; }) {
  return (
    <div>
      <span className="text-white/30">{label}</span>
      <span className="text-white/50 ml-4">{value}</span>
    </div>
  );
}

export function Info({ dapp, className }: Props) {
  return (
    <div className={clsx('grid grid-cols-1 gap-8 w-full', className)}>
      <p className="text-2xl font-semibold">{dapp.name}</p>

      <div>
        <p className="text-white/50">Published by</p>
        <p className="mt-4 text-primary-400">@{dapp.scope}</p>
      </div>

      <div>
        <p className="text-white/50">Repository</p>
        <a href={dapp.repositoryUrl} className="w-fit mt-4 text-white flex items-center gap-2.5" target="_blank" rel="noreferrer">
          <GitIcon width="15" height="15" />
          <span className="underline">{dapp.repositoryUrl.replace(/http(s)?:\/\//i, '')}</span>
        </a>
      </div>

      <div>
        <p className="text-white/50">Codegen</p>
        <p className="w-fit mt-4 px-8 py-4 flex items-center gap-3 font-roboto bg-gradient-to-b from-[#434343]/[0.35] to-transparent rounded-lg backdrop-blur-[14.605px]">
          <span>telchar codegen {dapp.scope}/{dapp.name}</span>
          <CopyIcon />
        </p>
      </div>

      <div className="flex flex-col gap-2">
        <CommonDetails
          label="Plutus version"
          value={dapp.blueprint.plutusVersion}
        />
        <CommonDetails
          label="Compiler"
          value={dapp.blueprint.compilerName + ' ' + dapp.blueprint.compilerVersion}
        />
        <CommonDetails
          label="License"
          value={dapp.blueprint.license}
        />
        <CommonDetails
          label="Publication date"
          value={`Published ${dapp.publishedDate ? dayjs(new Date(dapp.publishedDate * 1000)).fromNow() : ''}`}
        />
      </div>

    </div>
  );
}
