import clsx from 'clsx';

// https://cips.cardano.org/cip/CIP-0057
type ValidatorPurpose = 'spend' | 'mint' | 'withdraw' | 'publish';

type ValidatorSchema = {
  title?: string;
  description?: string;
  purpose?: 'spend' | 'mint' | 'withdraw' | 'publish' | ValidatorPurpose[];
  schema: { $ref: string; };
};

export type Validator = {
  title: string;
  redeemer?: ValidatorSchema;
  datum?: ValidatorSchema;
  parameters?: ValidatorSchema[];
  compiledCode?: string;
  hash?: string;
};

function InfoRow({ title, value, className }: { title: string; value: string | string[]; className?: string; }) {
  return (
    <>
      <div className={clsx('text-white/40', className)}>{title}</div>
      <div className="bg-white/40 w-[1px] h-full" />
      {Array.isArray(value)
        ? (
          <div className="flex flex-row gap-2.5 items-center flex-wrap">
            {
              value.map(param => (
                <span
                  key={param}
                  className="bg-white/5 rounded border border-white/20 px-1 py-0.5 text-white/60 font-roboto"
                >
                  {param}
                </span>
              ))
            }
          </div>
        )
        : (
          <span className="text-white/80 font-roboto">
            "{value}"
          </span>
        )}
    </>
  );
}

interface ValidatorInfoProps {
  validator: Validator;
}

export function ValidatorInfo({ validator }: ValidatorInfoProps) {
  return (
    <div className="grid grid-cols-[auto_auto_1fr] gap-x-2.5 gap-y-2 mt-6">
      {validator.datum && (
        <InfoRow title={validator.datum.title ?? 'datum'} value={validator.datum.schema.$ref} />
      )}
      {validator.redeemer && (
        <InfoRow title={validator.redeemer.title ?? 'reedemer'} value={validator.redeemer.schema.$ref} />
      )}
      {(validator.datum || validator.redeemer) && (
        <div className="col-span-3 h-4" />
      )}

      {(validator.parameters?.length ?? 0) > 0 && (
        <InfoRow
          title="parameters"
          value={validator.parameters!.map(p => p.title ?? p.schema.$ref)}
        />
      )}
    </div>
  );
}
