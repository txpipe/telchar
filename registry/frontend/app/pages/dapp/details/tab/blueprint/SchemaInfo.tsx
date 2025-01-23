import { useMemo } from 'react';

interface SchemaInfoProps {
  schema: DappSchema['schema'];
}

export function SchemaInfo({ schema }: SchemaInfoProps) {
  const parsedValue = useMemo(() => {
    try {
      const parsedJson = JSON.parse(schema);
      return JSON.stringify(parsedJson, null, 4);
    } catch {
      return null;
    }
  }, [schema]);

  return (
    <div className="mt-6 text-white/70">
      {parsedValue
        ? (
          <pre className="font-roboto whitespace-pre-wrap break-words">{parsedValue}</pre>
        )
        : 'Invalid JSON'}
    </div>
  );
}
