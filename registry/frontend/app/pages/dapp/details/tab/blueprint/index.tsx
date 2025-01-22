// Components
import { ExpandableCell } from '~/components/ExpandableCell';
import { Button } from '~/components/ui/Button';
import { ImportIcon } from '~/components/icons/import';
import { GitIcon } from '~/components/icons/git';

// Local
import { ValidatorInfo } from './ValidatorInfo';

interface Props {
  validators: DappValidator[];
  blueprintUrl: string;
}

export function TabBlueprint({ validators, blueprintUrl }: Props) {
  const downloadBlueprint = async () => {
    if (blueprintUrl) {
      const rawUrl = blueprintUrl
        .replace('github.com', 'raw.githubusercontent.com')
        .replace('/blob', '');

      const filename = blueprintUrl.split('/').pop() ?? 'plutus.json';

      try {
        const result = await fetch(rawUrl);
        if (!result.ok) {
          throw new Error('Failed to download blueprint');
        }
        const blobFile = await result.blob();
        const blobUrl = window.URL.createObjectURL(blobFile);

        const tmpLink = document.createElement('a');
        tmpLink.href = blobUrl;
        tmpLink.download = filename;
        document.body.appendChild(tmpLink);
        tmpLink.click();
        document.body.removeChild(tmpLink);
        window.URL.revokeObjectURL(blobUrl);
      } catch {}
    }
  };

  return (
    <div>
      <h2 className="text-xl font-semibold pb-1.5 border-b border-b-white/20">Blueprint file</h2>
      <div className="flex flex-row justify-between items-center mt-4">
        <a href={blueprintUrl} className="w-fit text-white flex items-center gap-2.5">
          <GitIcon width="15" height="15" />
          <span className="underline">{(blueprintUrl ?? '').replace(/http(s)?:\/\//, '')}</span>
        </a>
        <Button spacing="compact" text="large" weight="normal" outlined color="white" onClick={downloadBlueprint}>
          Download
          <ImportIcon />
        </Button>
      </div>
      <div className="flex flex-col gap-4 mt-8">
        <h2 className="text-xl font-semibold pb-1.5 border-b border-b-white/20">Validators</h2>
        {validators.map(validator => (
          <ExpandableCell key={validator.name} label={validator.name}>
            <ValidatorInfo validator={validator} />
          </ExpandableCell>
        ))}
      </div>
    </div>
  );
}
