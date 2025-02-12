import clsx from 'clsx';

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
  validator: DappValidator;
}

export function ValidatorInfo({ validator }: ValidatorInfoProps) {
  return (
    <div className="grid grid-cols-[auto_auto_1fr] gap-x-2.5 gap-y-2 mt-6">
      {validator.datum && (
        <InfoRow title="datum" value={validator.datum.schemaName} />
      )}
      {validator.redeemer && (
        <InfoRow title="reedemer" value={validator.redeemer.schemaName} />
      )}
      {(validator.datum || validator.redeemer) && (
        <div className="col-span-3 h-4" />
      )}

      {(validator.parameters?.length ?? 0) > 0 && (
        <InfoRow
          title="parameters"
          value={validator.parameters!.map(p => p.name ?? p.schemaName)}
        />
      )}
    </div>
  );
}
