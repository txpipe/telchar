import { request, type RequestDocument, type Variables } from 'graphql-request';
import type { VariablesAndRequestHeadersArgs } from 'node_modules/graphql-request/build/legacy/helpers/types';
import type { TypedDocumentNode } from '@graphql-typed-document-node/core';

const GRAPHQL_ENDPOINT = `${process.env.API_ENDPOINT}/graphql`;

export function requestGraphQL<T, V extends Variables = Variables>(
  document: RequestDocument | TypedDocumentNode<T, V>,
  ...variablesAndRequestHeaders: VariablesAndRequestHeadersArgs<V>
): () => Promise<T> {
  return async () => {
    return request<T, V>(
      GRAPHQL_ENDPOINT,
      document,
      ...variablesAndRequestHeaders,
    );
  };
}
