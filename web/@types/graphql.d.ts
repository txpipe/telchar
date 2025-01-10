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
  team: Maybe<Team>;
  teamId: Scalars['String']['output'];
}

interface Query {
  dapp: Maybe<Dapp>;
  dapps: Array<Dapp>;
  team: Maybe<Team>;
  teams: Array<Team>;
}

interface QueryDappArgs {
  id: Scalars['ID']['input'];
}

interface QueryTeamArgs {
  id: Scalars['ID']['input'];
}

interface Team {
  dapps: Array<Dapp>;
  id: Scalars['ID']['output'];
  name: Scalars['String']['output'];
  repository: Scalars['String']['output'];
}
