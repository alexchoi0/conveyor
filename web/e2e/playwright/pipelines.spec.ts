import { test, expect } from '@playwright/test';

test.describe('Pipelines Page', () => {
  test('should display pipelines page header', async ({ page }) => {
    await page.goto('/pipelines');
    await expect(page.locator('h1')).toContainText('Pipelines');
  });

  test('should display pipeline visualization section', async ({ page }) => {
    await page.goto('/pipelines');
    await expect(page.getByText('Pipeline Visualization')).toBeVisible();
  });

  test('should display pipelines list section', async ({ page }) => {
    await page.goto('/pipelines');
    await expect(page.getByText('Pipeline List')).toBeVisible();
  });

  test('should show no pipelines message when empty', async ({ page }) => {
    await page.goto('/pipelines');
    const noPipelinesMessage = page.getByText('No pipelines configured');
    const table = page.locator('table');
    await expect(noPipelinesMessage.or(table)).toBeVisible();
  });
});
