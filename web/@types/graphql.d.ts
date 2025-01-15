type Maybe<T> = T | null;
type InputMaybe<T> = Maybe<T>;
type Exact<T extends { [key: string]: unknown; }> = { [K in keyof T]: T[K] };
type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
type MakeEmpty<T extends { [key: string]: unknown; }, K extends keyof T> = { [_ in K]?: never };
type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
interface Scalars {
  ID: { input: string; output: string; };
  String: { input: string; output: string; };
  Boolean: { input: boolean; output: boolean; };
  Int: { input: number; output: number; };
  Float: { input: number; output: number; };
}

interface Dapp {
  description: Scalars['String']['output'];
  id: Scalars['ID']['output'];
  name: Scalars['String']['output'];
  publishedDate: Scalars['Int']['output'];
  repository: Scalars['String']['output'];
  scope: Maybe<Scope>;
  scopeId: Scalars['String']['output'];
}

interface DappConnection {
  /** A list of edges. */
  edges: Array<DappEdge>;
  metadata: Maybe<PaginationInfo>;
  /** A list of nodes. */
  nodes: Array<Dapp>;
  /** Information to aid in pagination. */
  pageInfo: PageInfo;
}

/** An edge in a connection. */
interface DappEdge {
  /** A cursor for use in pagination */
  cursor: Scalars['String']['output'];
  /** The item at the end of the edge */
  node: Dapp;
}

/** Information about pagination in a connection */
interface PageInfo {
  /** When paginating forwards, the cursor to continue. */
  endCursor: Maybe<Scalars['String']['output']>;
  /** When paginating forwards, are there more items? */
  hasNextPage: Scalars['Boolean']['output'];
  /** When paginating backwards, are there more items? */
  hasPreviousPage: Scalars['Boolean']['output'];
  /** When paginating backwards, the cursor to continue. */
  startCursor: Maybe<Scalars['String']['output']>;
}

interface PaginationInfo {
  pageSize: Scalars['Int']['output'];
  totalNodes: Scalars['Int']['output'];
}

interface Query {
  dapp: Maybe<Dapp>;
  dapps: DappConnection;
  scope: Maybe<Scope>;
  scopes: ScopeConnection;
}

interface QueryDappArgs {
  id: Scalars['ID']['input'];
}

interface QueryDappsArgs {
  after: InputMaybe<Scalars['String']['input']>;
  before: InputMaybe<Scalars['String']['input']>;
  first: InputMaybe<Scalars['Int']['input']>;
  last: InputMaybe<Scalars['Int']['input']>;
}

interface QueryScopeArgs {
  id: Scalars['ID']['input'];
}

interface QueryScopesArgs {
  after: InputMaybe<Scalars['String']['input']>;
  before: InputMaybe<Scalars['String']['input']>;
  first: InputMaybe<Scalars['Int']['input']>;
  last: InputMaybe<Scalars['Int']['input']>;
}

interface Scope {
  dapps: Array<Dapp>;
  id: Scalars['ID']['output'];
  name: Scalars['String']['output'];
  repository: Scalars['String']['output'];
}

interface ScopeConnection {
  /** A list of edges. */
  edges: Array<ScopeEdge>;
  metadata: Maybe<PaginationInfo>;
  /** A list of nodes. */
  nodes: Array<Scope>;
  /** Information to aid in pagination. */
  pageInfo: PageInfo;
}

/** An edge in a connection. */
interface ScopeEdge {
  /** A cursor for use in pagination */
  cursor: Scalars['String']['output'];
  /** The item at the end of the edge */
  node: Scope;
}
