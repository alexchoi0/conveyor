<script lang="ts">
  import { enhance } from '$app/forms';
  import {
    Card, CardHeader, CardTitle, CardDescription, CardContent,
    Table, TableHeader, TableBody, TableRow, TableHead, TableCell,
    Badge, Button
  } from '$lib/components/ui';
  import { Trash2, User } from 'lucide-svelte';

  interface PageData {
    users: {
      id: string;
      name: string | null;
      email: string;
      image: string | null;
      role: string;
      createdAt: Date;
    }[];
  }

  let { data, form }: { data: PageData; form: { error?: string; success?: boolean } | null } = $props();

  function getRoleBadgeVariant(role: string): 'default' | 'secondary' | 'destructive' {
    switch (role) {
      case 'SUPER_ADMIN':
        return 'destructive';
      case 'ADMIN':
        return 'default';
      default:
        return 'secondary';
    }
  }

  function formatDate(date: Date): string {
    return new Date(date).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
    });
  }

  let deletingUserId = $state<string | null>(null);
  let updatingUserId = $state<string | null>(null);
</script>

<div class="space-y-8">
  <div>
    <h1 class="text-3xl font-bold tracking-tight">User Management</h1>
    <p class="text-muted-foreground">Manage user accounts and roles</p>
  </div>

  {#if form?.error}
    <div class="rounded-lg bg-destructive/10 border border-destructive/20 p-4 text-sm text-destructive">
      {form.error}
    </div>
  {/if}

  {#if form?.success}
    <div class="rounded-lg bg-green-500/10 border border-green-500/20 p-4 text-sm text-green-600">
      Operation completed successfully
    </div>
  {/if}

  <Card>
    <CardHeader>
      <CardTitle>Users</CardTitle>
      <CardDescription>{data.users.length} registered user(s)</CardDescription>
    </CardHeader>
    <CardContent>
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>User</TableHead>
            <TableHead>Email</TableHead>
            <TableHead>Role</TableHead>
            <TableHead>Joined</TableHead>
            <TableHead class="text-right">Actions</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {#each data.users as user}
            <TableRow>
              <TableCell>
                <div class="flex items-center gap-3">
                  {#if user.image}
                    <img src={user.image} alt={user.name || 'User'} class="h-8 w-8 rounded-full" />
                  {:else}
                    <div class="flex h-8 w-8 items-center justify-center rounded-full bg-secondary">
                      <User class="h-4 w-4" />
                    </div>
                  {/if}
                  <span class="font-medium">{user.name || 'Unnamed'}</span>
                </div>
              </TableCell>
              <TableCell class="text-muted-foreground">{user.email}</TableCell>
              <TableCell>
                <form
                  method="POST"
                  action="?/updateRole"
                  use:enhance={() => {
                    updatingUserId = user.id;
                    return async ({ update }) => {
                      await update();
                      updatingUserId = null;
                    };
                  }}
                >
                  <input type="hidden" name="userId" value={user.id} />
                  <select
                    name="role"
                    class="h-8 rounded-md border bg-background px-2 text-sm"
                    value={user.role}
                    onchange={(e) => e.currentTarget.form?.requestSubmit()}
                    disabled={updatingUserId === user.id}
                  >
                    <option value="USER">User</option>
                    <option value="ADMIN">Admin</option>
                    <option value="SUPER_ADMIN">Super Admin</option>
                  </select>
                </form>
              </TableCell>
              <TableCell class="text-muted-foreground">{formatDate(user.createdAt)}</TableCell>
              <TableCell class="text-right">
                <form
                  method="POST"
                  action="?/deleteUser"
                  use:enhance={() => {
                    if (!confirm(`Are you sure you want to delete ${user.name || user.email}?`)) {
                      return () => {};
                    }
                    deletingUserId = user.id;
                    return async ({ update }) => {
                      await update();
                      deletingUserId = null;
                    };
                  }}
                >
                  <input type="hidden" name="userId" value={user.id} />
                  <Button
                    type="submit"
                    variant="ghost"
                    size="icon"
                    disabled={deletingUserId === user.id}
                  >
                    <Trash2 class="h-4 w-4 text-destructive" />
                  </Button>
                </form>
              </TableCell>
            </TableRow>
          {/each}
        </TableBody>
      </Table>
    </CardContent>
  </Card>
</div>
