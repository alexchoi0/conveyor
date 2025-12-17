// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces

import type { Session, User } from 'better-auth/types';
import type { Role } from '$lib/server/rbac';

interface AppUser extends User {
	role?: Role;
}

declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			user: AppUser | null;
			session: Session | null;
			authEnabled: boolean;
		}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}
}

export {};
