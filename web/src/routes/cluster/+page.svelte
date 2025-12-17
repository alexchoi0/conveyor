<script lang="ts">
  import { queryStore, getContextClient } from '@urql/svelte';
  import { CLUSTER_STATUS } from '$lib/graphql/queries';
  import {
    Card, CardHeader, CardTitle, CardDescription, CardContent,
    Badge, Button
  } from '$lib/components/ui';
  import { RefreshCw, Crown, Users, Database, GitCommit } from 'lucide-svelte';

  const client = getContextClient();
  const clusterQuery = queryStore({ client, query: CLUSTER_STATUS });

  function refresh() {
    clusterQuery.reexecute({ requestPolicy: 'network-only' });
  }

  function getRoleBadgeVariant(role: string): 'default' | 'secondary' | 'success' {
    switch (role?.toLowerCase()) {
      case 'leader':
        return 'success';
      case 'follower':
        return 'secondary';
      case 'candidate':
        return 'default';
      default:
        return 'secondary';
    }
  }

  function getRoleIcon(role: string) {
    switch (role?.toLowerCase()) {
      case 'leader':
        return Crown;
      case 'follower':
        return Users;
      default:
        return Database;
    }
  }
</script>

<div class="space-y-8">
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-3xl font-bold tracking-tight">Cluster</h1>
      <p class="text-muted-foreground">Raft cluster status and information</p>
    </div>
    <Button variant="outline" onclick={refresh}>
      <RefreshCw class="mr-2 h-4 w-4" />
      Refresh
    </Button>
  </div>

  {#if $clusterQuery.fetching}
    <Card>
      <CardContent class="py-8">
        <p class="text-center text-muted-foreground">Loading cluster status...</p>
      </CardContent>
    </Card>
  {:else if $clusterQuery.error}
    <Card>
      <CardContent class="py-8">
        <p class="text-center text-destructive">Failed to connect to cluster: {$clusterQuery.error.message}</p>
      </CardContent>
    </Card>
  {:else if !$clusterQuery.data?.clusterStatus}
    <Card>
      <CardContent class="py-8">
        <p class="text-center text-muted-foreground">Cluster status unavailable</p>
      </CardContent>
    </Card>
  {:else}
    {@const status = $clusterQuery.data.clusterStatus}
    {@const RoleIcon = getRoleIcon(status.role)}

    <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
      <Card>
        <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle class="text-sm font-medium">Node Role</CardTitle>
          <RoleIcon class="h-4 w-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <div class="flex items-center gap-2">
            <div class="text-2xl font-bold capitalize">{status.role}</div>
            <Badge variant={getRoleBadgeVariant(status.role)}>Active</Badge>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle class="text-sm font-medium">Current Term</CardTitle>
          <Database class="h-4 w-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <div class="text-2xl font-bold">{status.term}</div>
          <p class="text-xs text-muted-foreground">Election term</p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle class="text-sm font-medium">Commit Index</CardTitle>
          <GitCommit class="h-4 w-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <div class="text-2xl font-bold">{status.commitIndex}</div>
          <p class="text-xs text-muted-foreground">Last committed entry</p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle class="text-sm font-medium">Cluster Members</CardTitle>
          <Users class="h-4 w-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <div class="text-2xl font-bold">{status.memberCount}</div>
          <p class="text-xs text-muted-foreground">Total nodes</p>
        </CardContent>
      </Card>
    </div>

    <Card>
      <CardHeader>
        <CardTitle>Cluster Details</CardTitle>
        <CardDescription>Detailed Raft consensus information</CardDescription>
      </CardHeader>
      <CardContent>
        <div class="grid gap-6 md:grid-cols-2">
          <div class="space-y-4">
            <div>
              <p class="text-sm font-medium text-muted-foreground">Node ID</p>
              <p class="font-mono text-sm">{status.nodeId}</p>
            </div>

            <div>
              <p class="text-sm font-medium text-muted-foreground">Leader</p>
              {#if status.leader}
                <p class="font-mono text-sm">{status.leader}</p>
              {:else}
                <p class="text-sm text-muted-foreground italic">No leader elected</p>
              {/if}
            </div>

            <div>
              <p class="text-sm font-medium text-muted-foreground">Role</p>
              <Badge variant={getRoleBadgeVariant(status.role)} class="mt-1">
                {status.role}
              </Badge>
            </div>
          </div>

          <div class="space-y-4">
            <div>
              <p class="text-sm font-medium text-muted-foreground">Term</p>
              <p class="text-sm">{status.term}</p>
            </div>

            <div>
              <p class="text-sm font-medium text-muted-foreground">Commit Index</p>
              <p class="text-sm">{status.commitIndex}</p>
            </div>

            <div>
              <p class="text-sm font-medium text-muted-foreground">Last Applied</p>
              <p class="text-sm">{status.lastApplied}</p>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>

    <Card>
      <CardHeader>
        <CardTitle>Replication Status</CardTitle>
        <CardDescription>Log replication health</CardDescription>
      </CardHeader>
      <CardContent>
        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <span class="text-sm">Log entries committed</span>
            <span class="font-mono text-sm">{status.commitIndex}</span>
          </div>
          <div class="flex items-center justify-between">
            <span class="text-sm">Log entries applied</span>
            <span class="font-mono text-sm">{status.lastApplied}</span>
          </div>
          <div class="flex items-center justify-between">
            <span class="text-sm">Pending entries</span>
            <span class="font-mono text-sm">{status.commitIndex - status.lastApplied}</span>
          </div>

          {#if true}
            {@const pendingRatio = status.commitIndex > 0 ? status.lastApplied / status.commitIndex : 1}
            <div class="space-y-2">
              <div class="flex items-center justify-between text-sm">
                <span>Applied progress</span>
                <span>{Math.round(pendingRatio * 100)}%</span>
              </div>
              <div class="h-2 w-full rounded-full bg-secondary">
                <div
                  class="h-2 rounded-full bg-primary transition-all"
                  style="width: {pendingRatio * 100}%"
                ></div>
              </div>
            </div>
          {/if}
        </div>
      </CardContent>
    </Card>
  {/if}
</div>
