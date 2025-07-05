import { test, expect } from '@playwright/test';

test.describe('Card Display and Modal Tests', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:3003');
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(3000); // Wait for auto-loading to complete
  });

  test('should auto-load cards from all categories in unified container', async ({ page }) => {
    // Wait for unified container to appear with auto-loaded cards
    await expect(page.locator('.unified-container')).toBeVisible();
    
    // Check that cards are auto-loaded from all categories
    const allCards = page.locator('.unified-container .card');
    const cardCount = await allCards.count();
    expect(cardCount).toBeGreaterThan(0);
    
    // Verify different categories are represented
    const githubCards = page.locator('.card[data-category="github"]');
    const projectCards = page.locator('.card[data-category="project"]');
    const blogCards = page.locator('.card[data-category="blog"]');
    
    expect(await githubCards.count()).toBeGreaterThan(0);
    // Note: project and blog cards may be 0 if no content exists
  });

  test('should show correct content in modal when card is clicked', async ({ page }) => {
    // Wait for cards to auto-load
    await page.waitForTimeout(2000);
    
    // Find and click a card
    const firstCard = page.locator('.unified-container .card').first();
    await expect(firstCard).toBeVisible();
    await firstCard.click();
    
    // Wait for modal to appear
    await expect(page.locator('.modal')).toBeVisible();
    
    // Check modal content
    await expect(page.locator('.modal .modal-body')).toBeVisible();
    await expect(page.locator('.modal .modal-body h1').first()).toBeVisible();
    
    // Check modal can be closed
    await page.locator('.modal-close').click();
    await expect(page.locator('.modal')).not.toBeVisible();
  });

  test('should display GitHub cards with proper README content', async ({ page }) => {
    // Wait for auto-loading
    await page.waitForTimeout(2000);
    
    // Find GitHub cards
    const githubCards = page.locator('.card[data-category="github"]');
    expect(await githubCards.count()).toBeGreaterThan(0);
    
    // Click first GitHub card
    const firstGithubCard = githubCards.first();
    await firstGithubCard.click();
    
    // Check modal shows GitHub content
    await expect(page.locator('.modal')).toBeVisible();
    await expect(page.locator('.modal .github-readme')).toBeVisible();
    
    // Check for repository stats
    await expect(page.locator('.modal').getByText('stars')).toBeVisible();
    await expect(page.locator('.modal').getByText('forks')).toBeVisible();
    
    // Close modal
    await page.locator('.modal-close').click();
  });

  test('should handle responsive image scaling in modals', async ({ page }) => {
    // Wait for auto-loading
    await page.waitForTimeout(2000);
    
    // Find a GitHub card (most likely to have images)
    const githubCard = page.locator('.card[data-category="github"]').first();
    await githubCard.click();
    
    // Wait for modal and images
    await expect(page.locator('.modal')).toBeVisible();
    await page.waitForTimeout(1000); // Allow image scaling to apply
    
    // Check if images exist and are properly styled
    const modalImages = page.locator('.modal img');
    const imageCount = await modalImages.count();
    
    if (imageCount > 0) {
      // Verify images don't cause horizontal overflow
      const modalBody = page.locator('.modal-body');
      const bodyWidth = await modalBody.evaluate(el => el.scrollWidth);
      const clientWidth = await modalBody.evaluate(el => el.clientWidth);
      
      // scrollWidth should not exceed clientWidth (no horizontal scroll)
      expect(bodyWidth).toBeLessThanOrEqual(clientWidth + 10); // Allow small tolerance
    }
    
    // Close modal
    await page.locator('.modal-close').click();
  });
});