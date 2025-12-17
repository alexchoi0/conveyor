<script lang="ts">
  import { authClient } from '$lib/auth-client';
  import { Button } from '$lib/components/ui';
  import { LogOut, User } from 'lucide-svelte';
  import { goto } from '$app/navigation';

  const session = authClient.useSession();

  let isOpen = $state(false);

  async function signOut() {
    await authClient.signOut();
    goto('/auth/signin');
  }

  function toggleMenu() {
    isOpen = !isOpen;
  }

  function closeMenu() {
    isOpen = false;
  }
</script>

<svelte:window onclick={closeMenu} />

<div class="relative">
  <button
    class="flex items-center gap-2 rounded-full p-1 hover:bg-accent"
    onclick={(e) => { e.stopPropagation(); toggleMenu(); }}
  >
    {#if $session.data?.user?.image}
      <img
        src={$session.data.user.image}
        alt={$session.data.user.name || 'User'}
        class="h-8 w-8 rounded-full"
      />
    {:else}
      <div class="flex h-8 w-8 items-center justify-center rounded-full bg-primary text-primary-foreground">
        <User class="h-4 w-4" />
      </div>
    {/if}
  </button>

  {#if isOpen}
    <div
      class="absolute right-0 top-full mt-2 w-56 rounded-md border bg-background shadow-lg"
      onclick={(e) => e.stopPropagation()}
    >
      <div class="p-3 border-b">
        <p class="text-sm font-medium">{$session.data?.user?.name || 'User'}</p>
        <p class="text-xs text-muted-foreground">{$session.data?.user?.email}</p>
      </div>
      <div class="p-1">
        <button
          class="flex w-full items-center gap-2 rounded-md px-3 py-2 text-sm text-destructive hover:bg-accent"
          onclick={signOut}
        >
          <LogOut class="h-4 w-4" />
          Sign out
        </button>
      </div>
    </div>
  {/if}
</div>
