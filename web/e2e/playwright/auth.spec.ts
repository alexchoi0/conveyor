import { test, expect } from '@playwright/test';

test.describe('Authentication (disabled by default)', () => {
  test('should not show user menu when auth is disabled', async ({ page }) => {
    await page.goto('/');
    const userMenu = page.locator('[data-testid="user-menu"]');
    await expect(userMenu).not.toBeVisible();
  });

  test('should not show admin link when auth is disabled', async ({ page }) => {
    await page.goto('/');
    const adminLink = page.locator('a[href="/admin"]');
    await expect(adminLink).not.toBeVisible();
  });

  test('should allow access to all pages without login when auth is disabled', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('h1')).toContainText('Dashboard');

    await page.goto('/services');
    await expect(page.locator('h1')).toContainText('Services');

    await page.goto('/pipelines');
    await expect(page.locator('h1')).toContainText('Pipelines');

    await page.goto('/cluster');
    await expect(page.locator('h1')).toContainText('Cluster');

    await page.goto('/metrics');
    await expect(page.locator('h1')).toContainText('Metrics');
  });
});
