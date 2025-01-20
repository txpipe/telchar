import dayjs from 'dayjs';
import { useSearchParams } from 'react-router';

// Components
import { Header } from '~/components/Header';
import { DocumentIcon } from '~/components/icons/document';
import { InfoIcon } from '~/components/icons/info';
import { TabName } from '~/components/TabName';

// Local components
import { TabReadme } from './tab/readme';
import { TabBlueprint } from './tab/blueprint';
import { TabDeployment } from './tab/deployment';
import { Info } from './info';

// Types
type Tab = 'readme' | 'blueprint' | 'deployment';
const validTabs: Tab[] = ['readme', 'blueprint', 'deployment'];

function getValidTab(tab?: string): Tab {
  if (!tab) return 'readme';

  if (validTabs.includes(tab as Tab)) {
    return tab as Tab;
  }

  return 'readme';
}

export function DAppDetails({ dapp, readme }: { dapp: DappDetail; readme: string | null; }) {
  const [searchParams, setSearchParams] = useSearchParams({ activeTab: 'readme' });
  const activeTab: Tab = getValidTab(searchParams.get('activeTab')?.toLowerCase());
  return (
    <>
      <Header withoutNav withSearch />
      <main className="mt-20">
        <h1 className="text-2xl font-semibold">{dapp.name}</h1>
        <div className="mt-3 opacity-60">
          <h2 className="inline text-primary-400">@{dapp.scope}</h2>
          <span> • </span>
          <span>Published {dapp.publishedDate ? dayjs(new Date(dapp.publishedDate * 1000)).fromNow() : ''}</span>
        </div>

        <div className="flex mt-14 border-b-[#3E3E3E] border-b gap-6">
          <TabName
            icon={<InfoIcon width="14" height="14" />}
            name="Readme"
            active={activeTab === 'readme'}
            onClick={() => setSearchParams({ activeTab: 'readme' })}
          />
          <TabName
            icon={<DocumentIcon width="14" height="14" />}
            name="Blueprint"
            active={activeTab === 'blueprint'}
            onClick={() => setSearchParams({ activeTab: 'blueprint' })}
          />
          <TabName
            icon={<DocumentIcon width="14" height="14" />}
            name="Deployment"
            active={activeTab === 'deployment'}
            onClick={() => setSearchParams({ activeTab: 'deployment' })}
          />
        </div>

        <div className="flex gap-14 mt-14 items-start">
          <div className="w-full">
            {activeTab === 'readme' && <TabReadme readme={readme} />}
            {activeTab === 'blueprint' && <TabBlueprint />}
            {activeTab === 'deployment' && <TabDeployment />}
          </div>
          <Info dapp={dapp} className="max-w-[460px] sticky top-2" />
        </div>
      </main>
    </>
  );
}
