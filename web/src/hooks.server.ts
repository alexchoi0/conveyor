import { auth } from '$lib/server/auth';
import { svelteKitHandler } from 'better-auth/svelte-kit';
import type { Handle } from '@sveltejs/kit';
import { redirect, error } from '@sveltejs/kit';
import { sequence } from '@sveltejs/kit/hooks';
import { hasRole, type Role } from '$lib/server/rbac';
import { isAuthEnabled } from '$lib/server/settings';

const publicRoutes = ['/auth/signin', '/api/auth'];

const adminRoutes: { path: string; role: Role }[] = [
  { path: '/admin', role: 'ADMIN' },
];

const authHandler: Handle = async ({ event, resolve }) => {
  const authEnabled = await isAuthEnabled();
  event.locals.authEnabled = authEnabled;

  if (!authEnabled) {
    return resolve(event);
  }
  return svelteKitHandler({ event, resolve, auth, building: false });
};

const protectionHandler: Handle = async ({ event, resolve }) => {
  if (!event.locals.authEnabled) {
    return resolve(event);
  }

  const isPublicRoute = publicRoutes.some(
    (route) => event.url.pathname === route || event.url.pathname.startsWith(route + '/')
  );

  if (isPublicRoute) {
    return resolve(event);
  }

  const session = await auth.api.getSession({
    headers: event.request.headers,
  });

  if (!session) {
    throw redirect(303, '/auth/signin');
  }

  event.locals.user = session.user as typeof event.locals.user;
  event.locals.session = session.session;

  const adminRoute = adminRoutes.find(
    (route) =>
      event.url.pathname === route.path || event.url.pathname.startsWith(route.path + '/')
  );

  if (adminRoute) {
    const userRole = (session.user as { role?: string }).role;
    if (!hasRole(userRole, adminRoute.role)) {
      throw error(403, {
        message: 'Access denied. You do not have permission to view this page.',
      });
    }
  }

  return resolve(event);
};

export const handle = sequence(authHandler, protectionHandler);
