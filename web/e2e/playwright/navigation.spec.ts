import { test, expect } from '@playwright/test';

test.describe('Navigation', () => {
  test('should load the dashboard page', async ({ page }) => {
    await page.goto('/');
    await expect(page).toHaveTitle('ETL Router');
    await expect(page.locator('h1')).toContainText('Dashboard');
  });

  test('should navigate to services page', async ({ page }) => {
    await page.goto('/');
    await page.click('a[href="/services"]');
    await expect(page.locator('h1')).toContainText('Services');
  });

  test('should navigate to pipelines page', async ({ page }) => {
    await page.goto('/');
    await page.click('a[href="/pipelines"]');
    await expect(page.locator('h1')).toContainText('Pipelines');
  });

  test('should navigate to cluster page', async ({ page }) => {
    await page.goto('/');
    await page.click('a[href="/cluster"]');
    await expect(page.locator('h1')).toContainText('Cluster');
  });

  test('should navigate to metrics page', async ({ page }) => {
    await page.goto('/');
    await page.click('a[href="/metrics"]');
    await expect(page.locator('h1')).toContainText('Metrics');
  });

  test('should show navigation links', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('nav')).toBeVisible();
    await expect(page.locator('a[href="/"]')).toBeVisible();
    await expect(page.locator('a[href="/services"]')).toBeVisible();
    await expect(page.locator('a[href="/pipelines"]')).toBeVisible();
    await expect(page.locator('a[href="/cluster"]')).toBeVisible();
    await expect(page.locator('a[href="/metrics"]')).toBeVisible();
  });
});
