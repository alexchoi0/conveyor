import { prisma } from './db';

let cachedAuthEnabled: boolean | null = null;
let cacheTime = 0;
const CACHE_TTL = 5000;

export async function isAuthEnabled(): Promise<boolean> {
  const now = Date.now();
  if (cachedAuthEnabled !== null && now - cacheTime < CACHE_TTL) {
    return cachedAuthEnabled;
  }

  try {
    const settings = await prisma.settings.findUnique({
      where: { id: 'default' },
    });
    cachedAuthEnabled = settings?.authEnabled ?? false;
    cacheTime = now;
    return cachedAuthEnabled;
  } catch {
    return false;
  }
}

export async function setAuthEnabled(enabled: boolean): Promise<void> {
  await prisma.settings.upsert({
    where: { id: 'default' },
    update: { authEnabled: enabled },
    create: { id: 'default', authEnabled: enabled },
  });
  cachedAuthEnabled = enabled;
  cacheTime = Date.now();
}

export async function getSettings() {
  const settings = await prisma.settings.findUnique({
    where: { id: 'default' },
  });
  return settings ?? { id: 'default', authEnabled: false, createdAt: new Date(), updatedAt: new Date() };
}
