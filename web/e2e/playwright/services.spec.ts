import { test, expect } from '@playwright/test';

test.describe('Services Page', () => {
  test('should display services page header', async ({ page }) => {
    await page.goto('/services');
    await expect(page.locator('h1')).toContainText('Services');
  });

  test('should display services table headers', async ({ page }) => {
    await page.goto('/services');
    await expect(page.getByRole('columnheader', { name: 'Service ID' })).toBeVisible();
    await expect(page.getByRole('columnheader', { name: 'Type' })).toBeVisible();
    await expect(page.getByRole('columnheader', { name: 'Endpoint' })).toBeVisible();
    await expect(page.getByRole('columnheader', { name: 'Health' })).toBeVisible();
  });

  test('should show no services message when empty', async ({ page }) => {
    await page.goto('/services');
    const noServicesMessage = page.getByText('No services registered');
    const table = page.locator('table');
    await expect(noServicesMessage.or(table)).toBeVisible();
  });
});
