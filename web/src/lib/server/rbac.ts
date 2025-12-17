import type { RequestEvent } from '@sveltejs/kit';
import { redirect, error } from '@sveltejs/kit';
import { auth } from './auth';

export type Role = 'USER' | 'ADMIN' | 'SUPER_ADMIN';

const roleHierarchy: Record<Role, number> = {
  USER: 0,
  ADMIN: 1,
  SUPER_ADMIN: 2,
};

export function hasRole(userRole: string | undefined | null, requiredRole: Role): boolean {
  if (!userRole) return false;
  return (roleHierarchy[userRole as Role] ?? -1) >= roleHierarchy[requiredRole];
}

export function isAdmin(userRole: string | undefined | null): boolean {
  return hasRole(userRole, 'ADMIN');
}

export function isSuperAdmin(userRole: string | undefined | null): boolean {
  return hasRole(userRole, 'SUPER_ADMIN');
}

export async function requireAuth(event: RequestEvent) {
  const session = await auth.api.getSession({
    headers: event.request.headers,
  });

  if (!session) {
    throw redirect(303, '/auth/signin');
  }

  return session;
}

export async function requireRole(event: RequestEvent, requiredRole: Role) {
  const session = await requireAuth(event);

  const userRole = (session.user as { role?: string }).role;

  if (!hasRole(userRole, requiredRole)) {
    throw error(403, {
      message: `Access denied. Required role: ${requiredRole}`,
    });
  }

  return session;
}

export async function requireAdmin(event: RequestEvent) {
  return requireRole(event, 'ADMIN');
}

export async function requireSuperAdmin(event: RequestEvent) {
  return requireRole(event, 'SUPER_ADMIN');
}
