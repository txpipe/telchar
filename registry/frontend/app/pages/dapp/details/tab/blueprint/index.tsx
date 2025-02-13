// Components
import { ExpandableCell } from '~/components/ExpandableCell';
import { Button } from '~/components/ui/Button';
import { ImportIcon } from '~/components/icons/import';
import { GitIcon } from '~/components/icons/git';

// Local
import { ValidatorInfo } from './ValidatorInfo';
import { SchemaInfo } from './SchemaInfo';

interface Props {
  validators: DappValidator[];
  schemas: DappSchema[];
  repo: string;
}

export function TabBlueprint({ validators, schemas, repo }: Props) {
  const blueprintUrl = `${import.meta.env.VITE_MAIN_URL}/dapp/${repo}/raw/plutus.json`;

  const downloadBlueprint = async () => {
    try {
      const result = await fetch(blueprintUrl);
      if (!result.ok) {
        throw new Error('Failed to download blueprint');
      }
      const blobFile = await result.blob();
      const blobUrl = window.URL.createObjectURL(blobFile);

      const tmpLink = document.createElement('a');
      tmpLink.href = blobUrl;
      tmpLink.download = 'plutus.json';
      document.body.appendChild(tmpLink);
      tmpLink.click();
      document.body.removeChild(tmpLink);
      window.URL.revokeObjectURL(blobUrl);
    } catch {}
  };

  return (
    <div>
      <h2 className="text-xl font-semibold pb-1.5 border-b border-b-white/20">Blueprint file</h2>
      <div className="flex flex-row justify-between items-center mt-4">
        <a href={blueprintUrl} className="w-fit text-white flex items-center gap-2.5" target="_blank" rel="noreferrer">
          <GitIcon width="15" height="15" />
          <span className="underline">{blueprintUrl.replace(/^http(s)?:\/\//, '')}</span>
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

      <div className="flex flex-col gap-4 mt-8">
        <h2 className="text-xl font-semibold pb-1.5 border-b border-b-white/20">Schemas</h2>
        {schemas.map(schema => (
          <ExpandableCell key={schema.name} label={schema.name}>
            <SchemaInfo schema={schema.schema} />
          </ExpandableCell>
        ))}
      </div>
    </div>
  );
}
