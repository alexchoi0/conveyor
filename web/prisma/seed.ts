import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

async function main() {
  const superAdminEmail = process.env.SUPER_ADMIN_EMAIL;

  if (!superAdminEmail) {
    console.log('No SUPER_ADMIN_EMAIL set, skipping super admin setup');
    return;
  }

  const existingUser = await prisma.user.findUnique({
    where: { email: superAdminEmail },
  });

  if (existingUser) {
    if (existingUser.role !== 'SUPER_ADMIN') {
      await prisma.user.update({
        where: { email: superAdminEmail },
        data: { role: 'SUPER_ADMIN' },
      });
      console.log(`Promoted ${superAdminEmail} to SUPER_ADMIN`);
    } else {
      console.log(`${superAdminEmail} is already a SUPER_ADMIN`);
    }
  } else {
    console.log(`User ${superAdminEmail} not found. They will be promoted to SUPER_ADMIN on first sign-in.`);
  }
}

main()
  .catch((e) => {
    console.error(e);
    process.exit(1);
  })
  .finally(async () => {
    await prisma.$disconnect();
  });
