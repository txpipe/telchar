import { gql } from 'graphql-request';

export const DAPPS_QUERY = gql`
  query dapps {
    dapps {
      id
      name
      team {
        id
        name
      }
      description
    }
  }
`;
