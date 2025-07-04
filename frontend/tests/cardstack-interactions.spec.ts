import { test, expect } from '@playwright/test';

test.describe('CardStack Interactions', () => {
  test.beforeEach(async ({ page }) => {
    // Start with both frontend and backend running
    await page.goto('http://localhost:3003');
    
    // Wait for page to load
    await page.waitForLoadState('networkidle');
  });

  test('should have CardStack elements with correct data attributes', async ({ page }) => {
    // Check that CardStack elements exist
    const cardStacks = await page.locator('.card-stack').all();
    expect(cardStacks).toHaveLength(3);
    
    // Check data-category attributes
    await expect(page.locator('.card-stack[data-category="project"]')).toBeVisible();
    await expect(page.locator('.card-stack[data-category="blog"]')).toBeVisible();
    await expect(page.locator('.card-stack[data-category="github"]')).toBeVisible();
  });

  test('should initialize CardInteractions script', async ({ page }) => {
    // Wait for initialization message
    const initMessage = page.waitForEvent('console', msg => 
      msg.text().includes('CardInteractions initialized')
    );
    
    await initMessage;
    
    // Check that card stacks were found and initialized
    const foundMessage = page.waitForEvent('console', msg => 
      msg.text().includes('Found 3 card stacks to initialize')
    );
    
    await foundMessage;
  });

  test('should handle CardStack clicks and log debug info', async ({ page }) => {
    // Listen for console messages
    const consoleMessages: string[] = [];
    page.on('console', msg => consoleMessages.push(msg.text()));
    
    // Click on project CardStack
    await page.locator('.card-stack[data-category="project"]').click();
    
    // Wait for click to be processed
    await page.waitForTimeout(1000);
    
    // Check that click was logged
    expect(consoleMessages).toContainEqual(expect.stringContaining('CardStack clicked: project'));
    expect(consoleMessages).toContainEqual(expect.stringContaining('handleCardStackClick called with category: project'));
    expect(consoleMessages).toContainEqual(expect.stringContaining('Dealing cards from stack'));
  });

  test('should verify DOM elements during card dealing', async ({ page }) => {
    const consoleMessages: string[] = [];
    page.on('console', msg => consoleMessages.push(msg.text()));
    
    // Click on blog CardStack
    await page.locator('.card-stack[data-category="blog"]').click();
    
    // Wait for processing
    await page.waitForTimeout(2000);
    
    // Check DOM verification logs
    expect(consoleMessages).toContainEqual(expect.stringContaining('DOM ready: complete'));
    expect(consoleMessages).toContainEqual(expect.stringContaining('Grid element found: true'));
    expect(consoleMessages).toContainEqual(expect.stringContaining('Fetching from API:'));
  });

  test('should make API calls to backend', async ({ page }) => {
    // Monitor network requests
    const apiRequests: string[] = [];
    page.on('request', request => {
      if (request.url().includes('localhost:4000')) {
        apiRequests.push(request.url());
      }
    });
    
    // Click on project CardStack
    await page.locator('.card-stack[data-category="project"]').click();
    
    // Wait for API call
    await page.waitForTimeout(2000);
    
    // Check that API was called
    expect(apiRequests).toContainEqual(expect.stringContaining('/api/content/project'));
  });

  test('should create cards container in DOM', async ({ page }) => {
    // Click on blog CardStack
    await page.locator('.card-stack[data-category="blog"]').click();
    
    // Wait for cards to be created
    await page.waitForTimeout(3000);
    
    // Check if cards container was created
    const cardsContainer = page.locator('.cards-container[data-category="blog"]');
    await expect(cardsContainer).toBeAttached();
  });

  test('should toggle between dealing and returning cards', async ({ page }) => {
    const consoleMessages: string[] = [];
    page.on('console', msg => consoleMessages.push(msg.text()));
    
    const projectStack = page.locator('.card-stack[data-category="project"]');
    
    // First click - deal cards
    await projectStack.click();
    await page.waitForTimeout(1000);
    
    // Second click - return cards
    await projectStack.click();
    await page.waitForTimeout(1000);
    
    // Check logs for both actions
    expect(consoleMessages).toContainEqual(expect.stringContaining('Dealing cards from stack'));
    expect(consoleMessages).toContainEqual(expect.stringContaining('Returning cards to stack'));
  });

  test('should handle backend connectivity', async ({ page }) => {
    // Test if backend is reachable
    const response = await page.evaluate(async () => {
      try {
        const res = await fetch('http://localhost:4000/api/health');
        return { status: res.status, ok: res.ok };
      } catch (error) {
        return { error: error.message };
      }
    });
    
    expect(response.status).toBe(200);
    expect(response.ok).toBe(true);
  });
});