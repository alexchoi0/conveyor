import { test, expect } from '@playwright/test';

test.describe('Cluster Page', () => {
  test('should display cluster page header', async ({ page }) => {
    await page.goto('/cluster');
    await expect(page.locator('h1')).toContainText('Cluster');
  });

  test('should display cluster status section', async ({ page }) => {
    await page.goto('/cluster');
    await expect(page.getByText('Cluster Status')).toBeVisible();
  });

  test('should display node information', async ({ page }) => {
    await page.goto('/cluster');
    await expect(page.getByText('Node ID')).toBeVisible();
    await expect(page.getByText('Role')).toBeVisible();
    await expect(page.getByText('Term')).toBeVisible();
  });
});
