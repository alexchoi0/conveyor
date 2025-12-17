import { test, expect } from '@playwright/test';

test.describe('Metrics Page', () => {
  test('should display metrics page header', async ({ page }) => {
    await page.goto('/metrics');
    await expect(page.locator('h1')).toContainText('Metrics');
  });

  test('should display throughput card', async ({ page }) => {
    await page.goto('/metrics');
    await expect(page.getByText('Throughput')).toBeVisible();
  });

  test('should display consumer lag card', async ({ page }) => {
    await page.goto('/metrics');
    await expect(page.getByText('Consumer Lag')).toBeVisible();
  });

  test('should display services health card', async ({ page }) => {
    await page.goto('/metrics');
    await expect(page.getByText('Services Health')).toBeVisible();
  });

  test('should display active pipelines card', async ({ page }) => {
    await page.goto('/metrics');
    await expect(page.getByText('Active Pipelines').first()).toBeVisible();
  });

  test('should display throughput history section', async ({ page }) => {
    await page.goto('/metrics');
    await expect(page.getByText('Throughput History')).toBeVisible();
  });

  test('should display source offsets section', async ({ page }) => {
    await page.goto('/metrics');
    await expect(page.getByText('Source Offsets')).toBeVisible();
  });

  test('should display consumer lag details section', async ({ page }) => {
    await page.goto('/metrics');
    await expect(page.getByText('Consumer Lag Details')).toBeVisible();
  });

  test('should display consumer groups section', async ({ page }) => {
    await page.goto('/metrics');
    await expect(page.getByText('Consumer Groups').first()).toBeVisible();
  });

  test('should show last updated timestamp', async ({ page }) => {
    await page.goto('/metrics');
    await expect(page.getByText('Last updated:')).toBeVisible({ timeout: 5000 });
  });
});
