import { createClient, cacheExchange, fetchExchange } from '@urql/svelte';

export const client = createClient({
  url: 'http://localhost:8080/graphql',
  exchanges: [cacheExchange, fetchExchange],
});
