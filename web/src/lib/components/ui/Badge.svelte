<script lang="ts">
  import { cn } from '$lib/utils';
  import type { Snippet } from 'svelte';
  import type { HTMLAttributes } from 'svelte/elements';

  type Variant = 'default' | 'secondary' | 'destructive' | 'outline' | 'success';

  interface Props extends HTMLAttributes<HTMLDivElement> {
    variant?: Variant;
    children?: Snippet;
  }

  let { variant = 'default', class: className, children, ...restProps }: Props = $props();

  const variants: Record<Variant, string> = {
    default: 'border-transparent bg-primary text-primary-foreground',
    secondary: 'border-transparent bg-secondary text-secondary-foreground',
    destructive: 'border-transparent bg-destructive text-destructive-foreground',
    outline: 'text-foreground',
    success: 'border-transparent bg-green-500 text-white',
  };
</script>

<div
  class={cn(
    'inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors',
    variants[variant],
    className
  )}
  {...restProps}
>
  {#if children}{@render children()}{/if}
</div>
