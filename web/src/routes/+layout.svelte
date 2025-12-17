<script lang="ts">
  import '../app.css';
  import { setContextClient } from '@urql/svelte';
  import { client } from '$lib/graphql/client';
  import Nav from '$lib/components/Nav.svelte';

  let { children, data } = $props();

  setContextClient(client);
</script>

<svelte:head>
  <title>ETL Router</title>
</svelte:head>

<div class="min-h-screen bg-background">
  <Nav authEnabled={data.authEnabled} />
  <main class="container mx-auto px-6 py-8">
    {@render children()}
  </main>
</div>
