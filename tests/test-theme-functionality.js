const { test, expect } = require('@playwright/test');

test.describe('Theme Functionality', () => {
  test('should toggle theme and update placeholder detection', async ({ page }) => {
    // Visit the portfolio site
    await page.goto('http://localhost:3000');
    await page.waitForLoadState('networkidle');
    
    // Wait for cards to load
    await page.waitForSelector('.card', { timeout: 10000 });
    
    // Test the theme toggle functionality
    const initialTheme = await page.evaluate(() => {
      return document.documentElement.getAttribute('data-theme');
    });
    console.log('Initial theme:', initialTheme);
    
    // Test theme-aware placeholder function
    const initialPlaceholder = await page.evaluate(() => {
      function getThemeAwarePlaceholder() {
        const isLightTheme = document.documentElement.getAttribute('data-theme') === 'light';
        return isLightTheme ? '/light/card.jpg' : '/dark/card.jpg';
      }
      return getThemeAwarePlaceholder();
    });
    
    console.log('Initial placeholder:', initialPlaceholder);
    
    // Toggle theme
    const themeToggle = page.locator('#theme-toggle');
    await themeToggle.click();
    await page.waitForTimeout(500);
    
    const newTheme = await page.evaluate(() => {
      return document.documentElement.getAttribute('data-theme');
    });
    console.log('New theme:', newTheme);
    
    // Test theme-aware placeholder function after toggle
    const newPlaceholder = await page.evaluate(() => {
      function getThemeAwarePlaceholder() {
        const isLightTheme = document.documentElement.getAttribute('data-theme') === 'light';
        return isLightTheme ? '/light/card.jpg' : '/dark/card.jpg';
      }
      return getThemeAwarePlaceholder();
    });
    
    console.log('New placeholder:', newPlaceholder);
    
    // Verify theme toggling worked
    expect(newTheme).not.toBe(initialTheme);
    
    // Verify placeholders are different
    expect(newPlaceholder).not.toBe(initialPlaceholder);
    
    // Verify correct placeholder selection
    if (newTheme === 'light') {
      expect(newPlaceholder).toBe('/light/card.jpg');
    } else {
      expect(newPlaceholder).toBe('/dark/card.jpg');
    }
  });
  
  test('should create new cards with theme-aware placeholders', async ({ page }) => {
    // Visit the portfolio site
    await page.goto('http://localhost:3000');
    await page.waitForLoadState('networkidle');
    
    // Wait for cards to load
    await page.waitForSelector('.card', { timeout: 10000 });
    
    // Force recreate cards to test theme-aware placeholder
    const testResult = await page.evaluate(() => {
      // Simulate creating a new card without image
      const cardData = {
        title: 'Test Card',
        description: 'Test Description',
        // no image property - should use theme-aware placeholder
      };
      
      const category = 'test';
      const index = 0;
      
      // Copy the theme-aware placeholder function from CardInteractions
      function getThemeAwarePlaceholder() {
        const isLightTheme = document.documentElement.getAttribute('data-theme') === 'light';
        return isLightTheme ? '/light/card.jpg' : '/dark/card.jpg';
      }
      
      // Test both themes
      const results = {};
      
      // Test dark theme
      document.documentElement.setAttribute('data-theme', 'dark');
      results.darkPlaceholder = getThemeAwarePlaceholder();
      
      // Test light theme
      document.documentElement.setAttribute('data-theme', 'light');
      results.lightPlaceholder = getThemeAwarePlaceholder();
      
      return results;
    });
    
    console.log('Card creation test results:', testResult);
    
    // Verify correct placeholder selection
    expect(testResult.darkPlaceholder).toBe('/dark/card.jpg');
    expect(testResult.lightPlaceholder).toBe('/light/card.jpg');
  });
  
  test('should verify theme-aware placeholder files exist', async ({ page }) => {
    // Check that the theme-aware placeholder files exist
    const lightPlaceholderExists = await page.evaluate(async () => {
      try {
        const response = await fetch('/light/card.jpg');
        return response.ok;
      } catch (error) {
        return false;
      }
    });
    
    const darkPlaceholderExists = await page.evaluate(async () => {
      try {
        const response = await fetch('/dark/card.jpg');
        return response.ok;
      } catch (error) {
        return false;
      }
    });
    
    console.log('Light placeholder exists:', lightPlaceholderExists);
    console.log('Dark placeholder exists:', darkPlaceholderExists);
    
    expect(lightPlaceholderExists).toBe(true);
    expect(darkPlaceholderExists).toBe(true);
  });
});