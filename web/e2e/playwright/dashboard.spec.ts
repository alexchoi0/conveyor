import { test, expect } from '@playwright/test';

test.describe('Dashboard', () => {
  test('should display cluster status card', async ({ page }) => {
    await page.goto('/');
    await expect(page.getByText('Cluster Status')).toBeVisible();
  });

  test('should display services card', async ({ page }) => {
    await page.goto('/');
    await expect(page.getByText('Services').first()).toBeVisible();
  });

  test('should display pipelines card', async ({ page }) => {
    await page.goto('/');
    await expect(page.getByText('Pipelines').first()).toBeVisible();
  });

  test('should display members card', async ({ page }) => {
    await page.goto('/');
    await expect(page.getByText('Members')).toBeVisible();
  });

  test('should display recent services section', async ({ page }) => {
    await page.goto('/');
    await expect(page.getByText('Recent Services')).toBeVisible();
  });

  test('should display active pipelines section', async ({ page }) => {
    await page.goto('/');
    await expect(page.getByText('Active Pipelines')).toBeVisible();
  });
});
