import { CodegenConfig } from '@graphql-codegen/cli';

const config: CodegenConfig = {
  schema: './schema.graphql',
  // ignoreNoDocuments: true,
  generates: {
    '@types/graphql.d.ts': {
      plugins: ['typescript'],
      config: {
        skipTypename: true,
        enumsAsTypes: true,
        avoidOptionals: true,
        noExport: true,
        declarationKind: 'interface',
      },
    },
  },
  hooks: { afterAllFileWrite: ['eslint --fix'] },
};

export default config;
