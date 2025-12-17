import type { PageServerLoad, Actions } from './$types';
import { prisma } from '$lib/server/db';
import { requireAdmin } from '$lib/server/rbac';
import { fail } from '@sveltejs/kit';

export const load: PageServerLoad = async (event) => {
  await requireAdmin(event);

  const users = await prisma.user.findMany({
    orderBy: { createdAt: 'desc' },
    select: {
      id: true,
      name: true,
      email: true,
      image: true,
      role: true,
      createdAt: true,
    },
  });

  return { users };
};

export const actions: Actions = {
  updateRole: async (event) => {
    const session = await requireAdmin(event);
    const formData = await event.request.formData();
    const userId = formData.get('userId') as string;
    const newRole = formData.get('role') as string;

    if (!userId || !newRole) {
      return fail(400, { error: 'Missing required fields' });
    }

    if (!['USER', 'ADMIN', 'SUPER_ADMIN'].includes(newRole)) {
      return fail(400, { error: 'Invalid role' });
    }

    const currentUserRole = (session.user as { role?: string }).role;

    if (newRole === 'SUPER_ADMIN' && currentUserRole !== 'SUPER_ADMIN') {
      return fail(403, { error: 'Only super admins can assign super admin role' });
    }

    if (userId === session.user.id) {
      return fail(400, { error: 'You cannot change your own role' });
    }

    try {
      await prisma.user.update({
        where: { id: userId },
        data: { role: newRole },
      });

      return { success: true };
    } catch (e) {
      return fail(500, { error: 'Failed to update user role' });
    }
  },

  deleteUser: async (event) => {
    const session = await requireAdmin(event);
    const formData = await event.request.formData();
    const userId = formData.get('userId') as string;

    if (!userId) {
      return fail(400, { error: 'Missing user ID' });
    }

    if (userId === session.user.id) {
      return fail(400, { error: 'You cannot delete yourself' });
    }

    const targetUser = await prisma.user.findUnique({
      where: { id: userId },
      select: { role: true },
    });

    const currentUserRole = (session.user as { role?: string }).role;

    if (targetUser?.role === 'SUPER_ADMIN' && currentUserRole !== 'SUPER_ADMIN') {
      return fail(403, { error: 'Only super admins can delete super admin users' });
    }

    try {
      await prisma.user.delete({
        where: { id: userId },
      });

      return { success: true };
    } catch (e) {
      return fail(500, { error: 'Failed to delete user' });
    }
  },
};
