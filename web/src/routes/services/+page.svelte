<script lang="ts">
  import { queryStore, getContextClient, mutationStore } from '@urql/svelte';
  import { SERVICES } from '$lib/graphql/queries';
  import { DEREGISTER_SERVICE } from '$lib/graphql/mutations';
  import {
    Card, CardHeader, CardTitle, CardDescription, CardContent,
    Table, TableHeader, TableBody, TableRow, TableHead, TableCell,
    Badge, Button
  } from '$lib/components/ui';
  import { RefreshCw, Trash2 } from 'lucide-svelte';

  const client = getContextClient();
  const servicesQuery = queryStore({ client, query: SERVICES });

  function refresh() {
    servicesQuery.reexecute({ requestPolicy: 'network-only' });
  }

  async function deregister(serviceId: string) {
    if (!confirm(`Are you sure you want to deregister service "${serviceId}"?`)) {
      return;
    }

    const result = mutationStore({
      client,
      query: DEREGISTER_SERVICE,
      variables: { serviceId }
    });

    result.subscribe(res => {
      if (res.data) {
        refresh();
      }
    });
  }

  function formatDate(dateStr: string): string {
    try {
      return new Date(dateStr).toLocaleString();
    } catch {
      return dateStr;
    }
  }
</script>

<div class="space-y-8">
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-3xl font-bold tracking-tight">Services</h1>
      <p class="text-muted-foreground">Manage registered ETL services</p>
    </div>
    <Button variant="outline" onclick={refresh}>
      <RefreshCw class="mr-2 h-4 w-4" />
      Refresh
    </Button>
  </div>

  <Card>
    <CardHeader>
      <CardTitle>Registered Services</CardTitle>
      <CardDescription>
        {#if $servicesQuery.data?.services}
          {$servicesQuery.data.services.length} service(s) registered
        {/if}
      </CardDescription>
    </CardHeader>
    <CardContent>
      {#if $servicesQuery.fetching}
        <p class="text-muted-foreground">Loading services...</p>
      {:else if $servicesQuery.error}
        <p class="text-destructive">Failed to load services: {$servicesQuery.error.message}</p>
      {:else if !$servicesQuery.data?.services?.length}
        <p class="text-muted-foreground">No services registered yet.</p>
      {:else}
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Service ID</TableHead>
              <TableHead>Type</TableHead>
              <TableHead>Endpoint</TableHead>
              <TableHead>Health</TableHead>
              <TableHead>Registered</TableHead>
              <TableHead>Last Heartbeat</TableHead>
              <TableHead class="text-right">Actions</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {#each $servicesQuery.data.services as service}
              <TableRow>
                <TableCell class="font-medium">{service.serviceId}</TableCell>
                <TableCell>
                  <Badge variant="secondary">{service.serviceType}</Badge>
                </TableCell>
                <TableCell class="font-mono text-sm">{service.endpoint}</TableCell>
                <TableCell>
                  <Badge variant={service.healthStatus === 'healthy' ? 'success' : 'destructive'}>
                    {service.healthStatus}
                  </Badge>
                </TableCell>
                <TableCell class="text-sm text-muted-foreground">
                  {formatDate(service.registeredAt)}
                </TableCell>
                <TableCell class="text-sm text-muted-foreground">
                  {formatDate(service.lastHeartbeat)}
                </TableCell>
                <TableCell class="text-right">
                  <Button variant="ghost" size="icon" onclick={() => deregister(service.serviceId)}>
                    <Trash2 class="h-4 w-4 text-destructive" />
                  </Button>
                </TableCell>
              </TableRow>
            {/each}
          </TableBody>
        </Table>
      {/if}
    </CardContent>
  </Card>
</div>
