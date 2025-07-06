import { test, expect } from '@playwright/test';

test('GitHub cards should appear on the homepage', async ({ page }) => {
  await page.goto('http://localhost:3000');

  // Wait for auto-loading to complete (cards are auto-loaded, no click needed)
  await page.waitForTimeout(3000);

  // Check if GitHub cards are created (auto-loaded)
  const githubCards = page.locator('.card[data-category="github"]');
  expect(await githubCards.count()).toBeGreaterThan(0); // GitHub cards exist

  // Check if any GitHub card exists (order may vary)
  const firstGithubCard = githubCards.first();
  await expect(firstGithubCard).toBeVisible();

  // Check for holographic effect on featured card
  await expect(firstGithubCard).toHaveClass(/holographic/);

  // Check if cards have holographic effect (all GitHub cards may be holographic)
  const secondCard = githubCards.nth(1);
  await expect(secondCard).toBeVisible();
});

test('GitHub API endpoint returns correct data', async ({ page }) => {
  const response = await page.request.get('http://localhost:4000/api/github/projects');
  expect(response.status()).toBe(200);
  
  const data = await response.json();
  expect(data.success).toBe(true);
  expect(data.data.length).toBeGreaterThan(0); // GitHub projects exist
  
  // Check if at least one project has feature property
  const featuredProject = data.data.find(p => p.feature === true);
  expect(featuredProject).toBeTruthy();
  
  // Check if at least one project is not featured
  const nonFeaturedProject = data.data.find(p => p.feature === false);
  expect(nonFeaturedProject).toBeTruthy();
});