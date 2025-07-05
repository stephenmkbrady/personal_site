import { test, expect } from '@playwright/test';

test('GitHub cards should appear on the homepage', async ({ page }) => {
  await page.goto('http://localhost:3003');

  // Wait for auto-loading to complete (cards are auto-loaded, no click needed)
  await page.waitForTimeout(3000);

  // Check if GitHub cards are created (auto-loaded)
  const githubCards = page.locator('.card[data-category="github"]');
  await expect(githubCards).toHaveCount(3);

  // Check if VSCode card (featured) appears first
  const firstGithubCard = githubCards.first();
  await expect(firstGithubCard).toContainText('Visual Studio Code');

  // Check for holographic effect on featured card
  await expect(firstGithubCard).toHaveClass(/holographic/);

  // Check if non-featured cards don't have holographic effect
  const secondCard = githubCards.nth(1);
  await expect(secondCard).not.toHaveClass(/holographic/);
});

test('GitHub API endpoint returns correct data', async ({ page }) => {
  const response = await page.request.get('http://localhost:4000/api/github/projects');
  expect(response.status()).toBe(200);
  
  const data = await response.json();
  expect(data.success).toBe(true);
  expect(data.data).toHaveLength(3);
  
  // Check if VSCode is featured
  const vscodeProject = data.data.find(p => p.repo === 'vscode');
  expect(vscodeProject).toBeTruthy();
  expect(vscodeProject.feature).toBe(true);
  
  // Check if other projects are not featured
  const helloWorldProject = data.data.find(p => p.repo === 'Hello-World');
  expect(helloWorldProject.feature).toBe(false);
});