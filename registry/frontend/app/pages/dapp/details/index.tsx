import { Link, useSearchParams } from 'react-router';

// Components
import { DocumentIcon } from '~/components/icons/document';
// import { DeploymentIcon } from '~/components/icons/deployment';
import { InfoIcon } from '~/components/icons/info';
import { TabName } from '~/components/TabName';
import { SearchBar } from '~/components/SearchBar';
import { ToTopButton } from '~/components/ToTopButton';
import { Header } from '~/components/Header';

// Local components
import { ArrowLeftIcon } from '~/components/icons/arrow-left';
import { TabReadme } from './tab/readme';
import { TabBlueprint } from './tab/blueprint';
// import { TabDeployment } from './tab/deployment';
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

export function DAppDetails({ dapp, readme }: { dapp: Dapp; readme: string | null; }) {
  const [searchParams, setSearchParams] = useSearchParams({ activeTab: 'readme' });
  const activeTab: Tab = getValidTab(searchParams.get('activeTab')?.toLowerCase());

  return (
    <>
      <Header
        appName="Registry"
        centerNode={<SearchBar className="md:max-w-[500px]" dark />}
      />
      <main className="mt-20">
        <h1 className="text-3xl font-semibold mt-20">{dapp.name}</h1>
        <div className="mt-2 text-xl">
          <h2 className="inline text-primary-400">@{dapp.scope}</h2>
          <span className="text-white/60"> • v{dapp.blueprint.version}</span>
        </div>
        <p className="text-white/30 mt-6">{dapp.blueprint.description}</p>

        <div className="flex mt-14 border-b-[#3E3E3E] border-b gap-8">
          <TabName
            icon={<InfoIcon width="18" height="18" gradient={activeTab === 'readme' ? 'secondary' : undefined} />}
            name="Readme"
            active={activeTab === 'readme'}
            onClick={() => setSearchParams({ activeTab: 'readme' })}
          />
          <TabName
            icon={<DocumentIcon width="18" height="18" gradient={activeTab === 'blueprint' ? 'secondary' : undefined} />}
            name="Blueprint"
            active={activeTab === 'blueprint'}
            onClick={() => setSearchParams({ activeTab: 'blueprint' })}
          />
          {/* <TabName
            icon={<DeploymentIcon width="18" height="18" gradient={activeTab === 'deployment' ? 'secondary' : undefined} />}
            name="Deployment"
            active={activeTab === 'deployment'}
            onClick={() => setSearchParams({ activeTab: 'deployment' })}
          /> */}
        </div>

        <div className="flex gap-14 mt-8 items-start">
          <div className="w-full">
            {activeTab === 'readme' && <TabReadme readme={readme} />}
            {activeTab === 'blueprint' && (
              <TabBlueprint
                validators={dapp.blueprint.validators}
                schemas={dapp.blueprint.schemas}
                repo={dapp.id}
              />
            )}
            {/* {activeTab === 'deployment' && <TabDeployment />} */}
          </div>
          <Info dapp={dapp} className="max-w-[460px] sticky top-8" />
        </div>
        <ToTopButton />

        <Link to="/" className="flex items-center gap-3 w-fit text-white/80 hover:underline hover:underline-offset-[3px] hover:text-primary-400 mt-14">
          <ArrowLeftIcon width="18" height="18" /> Back to all dApps
        </Link>
      </main>
    </>
  );
}
