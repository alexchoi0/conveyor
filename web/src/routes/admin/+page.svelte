<script lang="ts">
  import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '$lib/components/ui';
  import { Users, Shield, Settings } from 'lucide-svelte';

  const adminLinks = [
    {
      href: '/admin/users',
      title: 'User Management',
      description: 'Manage users, roles, and permissions',
      icon: Users,
    },
    {
      href: '/admin/roles',
      title: 'Roles & Permissions',
      description: 'Configure role hierarchies and access levels',
      icon: Shield,
    },
    {
      href: '/admin/settings',
      title: 'System Settings',
      description: 'Configure application settings',
      icon: Settings,
    },
  ];
</script>

<div class="space-y-8">
  <div>
    <h1 class="text-3xl font-bold tracking-tight">Admin Dashboard</h1>
    <p class="text-muted-foreground">Manage users, roles, and system settings</p>
  </div>

  <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
    {#each adminLinks as link}
      <a href={link.href} class="block">
        <Card class="h-full transition-colors hover:bg-accent/50">
          <CardHeader>
            <div class="flex items-center gap-4">
              <div class="flex h-12 w-12 items-center justify-center rounded-lg bg-primary/10">
                <link.icon class="h-6 w-6 text-primary" />
              </div>
              <div>
                <CardTitle class="text-lg">{link.title}</CardTitle>
                <CardDescription>{link.description}</CardDescription>
              </div>
            </div>
          </CardHeader>
        </Card>
      </a>
    {/each}
  </div>
</div>
