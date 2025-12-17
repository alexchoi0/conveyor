<script lang="ts">
  import { SvelteFlow, Background, Controls, MiniMap, type Node, type Edge } from '@xyflow/svelte';
  import '@xyflow/svelte/dist/style.css';

  interface Pipeline {
    pipelineId: string;
    name: string;
    enabled: boolean;
    sourceId: string;
    sinkId: string;
    processors: string[];
  }

  interface Props {
    pipeline: Pipeline;
  }

  let { pipeline }: Props = $props();

  const nodes: Node[] = $derived.by(() => {
    const result: Node[] = [];
    let x = 0;
    const y = 100;
    const spacing = 200;

    result.push({
      id: 'source',
      type: 'input',
      data: { label: pipeline.sourceId },
      position: { x, y },
      style: 'background: #10b981; color: white; border-radius: 8px; padding: 10px;',
    });
    x += spacing;

    if (pipeline.processors && pipeline.processors.length > 0) {
      pipeline.processors.forEach((proc, i) => {
        result.push({
          id: `processor-${i}`,
          data: { label: proc },
          position: { x, y },
          style: 'background: #6366f1; color: white; border-radius: 8px; padding: 10px;',
        });
        x += spacing;
      });
    }

    result.push({
      id: 'sink',
      type: 'output',
      data: { label: pipeline.sinkId },
      position: { x, y },
      style: 'background: #f59e0b; color: white; border-radius: 8px; padding: 10px;',
    });

    return result;
  });

  const edges: Edge[] = $derived.by(() => {
    const result: Edge[] = [];
    const processorCount = pipeline.processors?.length ?? 0;

    if (processorCount === 0) {
      result.push({
        id: 'source-sink',
        source: 'source',
        target: 'sink',
        animated: pipeline.enabled,
        style: 'stroke-width: 2px;',
      });
    } else {
      result.push({
        id: 'source-proc-0',
        source: 'source',
        target: 'processor-0',
        animated: pipeline.enabled,
        style: 'stroke-width: 2px;',
      });

      for (let i = 0; i < processorCount - 1; i++) {
        result.push({
          id: `proc-${i}-${i + 1}`,
          source: `processor-${i}`,
          target: `processor-${i + 1}`,
          animated: pipeline.enabled,
          style: 'stroke-width: 2px;',
        });
      }

      result.push({
        id: `proc-${processorCount - 1}-sink`,
        source: `processor-${processorCount - 1}`,
        target: 'sink',
        animated: pipeline.enabled,
        style: 'stroke-width: 2px;',
      });
    }

    return result;
  });
</script>

<div class="h-[300px] w-full rounded-lg border bg-slate-50">
  <SvelteFlow {nodes} {edges} fitView>
    <Background />
    <Controls />
    <MiniMap />
  </SvelteFlow>
</div>
