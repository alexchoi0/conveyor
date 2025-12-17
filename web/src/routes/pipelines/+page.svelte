<script lang="ts">
  import { queryStore, getContextClient, mutationStore } from '@urql/svelte';
  import { PIPELINES } from '$lib/graphql/queries';
  import { ENABLE_PIPELINE, DISABLE_PIPELINE, DELETE_PIPELINE } from '$lib/graphql/mutations';
  import {
    Card, CardHeader, CardTitle, CardDescription, CardContent,
    Badge, Button, Switch
  } from '$lib/components/ui';
  import PipelineFlow from '$lib/components/PipelineFlow.svelte';
  import { RefreshCw, Trash2, ChevronDown, ChevronUp } from 'lucide-svelte';

  const client = getContextClient();
  const pipelinesQuery = queryStore({ client, query: PIPELINES });

  let expandedPipelines: Set<string> = $state(new Set());

  function refresh() {
    pipelinesQuery.reexecute({ requestPolicy: 'network-only' });
  }

  function toggleExpand(pipelineId: string) {
    if (expandedPipelines.has(pipelineId)) {
      expandedPipelines.delete(pipelineId);
    } else {
      expandedPipelines.add(pipelineId);
    }
    expandedPipelines = new Set(expandedPipelines);
  }

  async function togglePipeline(pipelineId: string, currentEnabled: boolean) {
    const query = currentEnabled ? DISABLE_PIPELINE : ENABLE_PIPELINE;
    const result = mutationStore({
      client,
      query,
      variables: { pipelineId }
    });

    result.subscribe(res => {
      if (res.data) {
        refresh();
      }
    });
  }

  async function deletePipeline(pipelineId: string) {
    if (!confirm(`Are you sure you want to delete pipeline "${pipelineId}"?`)) {
      return;
    }

    const result = mutationStore({
      client,
      query: DELETE_PIPELINE,
      variables: { pipelineId }
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
      <h1 class="text-3xl font-bold tracking-tight">Pipelines</h1>
      <p class="text-muted-foreground">Manage ETL pipeline configurations</p>
    </div>
    <Button variant="outline" onclick={refresh}>
      <RefreshCw class="mr-2 h-4 w-4" />
      Refresh
    </Button>
  </div>

  {#if $pipelinesQuery.fetching}
    <Card>
      <CardContent class="py-8">
        <p class="text-center text-muted-foreground">Loading pipelines...</p>
      </CardContent>
    </Card>
  {:else if $pipelinesQuery.error}
    <Card>
      <CardContent class="py-8">
        <p class="text-center text-destructive">Failed to load pipelines: {$pipelinesQuery.error.message}</p>
      </CardContent>
    </Card>
  {:else if !$pipelinesQuery.data?.pipelines?.length}
    <Card>
      <CardContent class="py-8">
        <p class="text-center text-muted-foreground">No pipelines configured yet.</p>
      </CardContent>
    </Card>
  {:else}
    <div class="space-y-4">
      {#each $pipelinesQuery.data.pipelines as pipeline}
        <Card>
          <CardHeader>
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-4">
                <Switch
                  checked={pipeline.enabled}
                  onchange={() => togglePipeline(pipeline.pipelineId, pipeline.enabled)}
                />
                <div>
                  <CardTitle class="text-lg">{pipeline.name || pipeline.pipelineId}</CardTitle>
                  <CardDescription>
                    {pipeline.sourceId} → {pipeline.processors?.length ? `${pipeline.processors.length} processors → ` : ''}{pipeline.sinkId}
                  </CardDescription>
                </div>
              </div>
              <div class="flex items-center gap-2">
                <Badge variant={pipeline.enabled ? 'success' : 'secondary'}>
                  {pipeline.enabled ? 'Enabled' : 'Disabled'}
                </Badge>
                <Button variant="ghost" size="icon" onclick={() => toggleExpand(pipeline.pipelineId)}>
                  {#if expandedPipelines.has(pipeline.pipelineId)}
                    <ChevronUp class="h-4 w-4" />
                  {:else}
                    <ChevronDown class="h-4 w-4" />
                  {/if}
                </Button>
                <Button variant="ghost" size="icon" onclick={() => deletePipeline(pipeline.pipelineId)}>
                  <Trash2 class="h-4 w-4 text-destructive" />
                </Button>
              </div>
            </div>
          </CardHeader>

          {#if expandedPipelines.has(pipeline.pipelineId)}
            <CardContent class="space-y-6">
              <div class="grid gap-4 md:grid-cols-3">
                <div>
                  <p class="text-sm font-medium text-muted-foreground">Pipeline ID</p>
                  <p class="font-mono text-sm">{pipeline.pipelineId}</p>
                </div>
                <div>
                  <p class="text-sm font-medium text-muted-foreground">Created</p>
                  <p class="text-sm">{formatDate(pipeline.createdAt)}</p>
                </div>
                <div>
                  <p class="text-sm font-medium text-muted-foreground">Updated</p>
                  <p class="text-sm">{formatDate(pipeline.updatedAt)}</p>
                </div>
              </div>

              {#if pipeline.processors?.length}
                <div>
                  <p class="text-sm font-medium text-muted-foreground mb-2">Processors</p>
                  <div class="flex flex-wrap gap-2">
                    {#each pipeline.processors as processor}
                      <Badge variant="secondary">{processor}</Badge>
                    {/each}
                  </div>
                </div>
              {/if}

              <div>
                <p class="text-sm font-medium text-muted-foreground mb-2">Pipeline Flow</p>
                <PipelineFlow {pipeline} />
              </div>
            </CardContent>
          {/if}
        </Card>
      {/each}
    </div>
  {/if}
</div>
