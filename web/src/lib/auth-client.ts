import { createAuthClient } from 'better-auth/svelte';
import { adminClient } from 'better-auth/client/plugins';

export const authClient = createAuthClient({
  baseURL: import.meta.env.VITE_BETTER_AUTH_URL || 'http://localhost:5173',
  plugins: [adminClient()],
});

export type Role = 'USER' | 'ADMIN' | 'SUPER_ADMIN';

export function hasRole(userRole: string | undefined, requiredRole: Role): boolean {
  if (!userRole) return false;

  const roleHierarchy: Record<Role, number> = {
    USER: 0,
    ADMIN: 1,
    SUPER_ADMIN: 2,
  };

  return (roleHierarchy[userRole as Role] ?? -1) >= roleHierarchy[requiredRole];
}

export function isAdmin(userRole: string | undefined): boolean {
  return hasRole(userRole, 'ADMIN');
}

export function isSuperAdmin(userRole: string | undefined): boolean {
  return hasRole(userRole, 'SUPER_ADMIN');
}
