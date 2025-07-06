import { test, expect } from '@playwright/test';

test.describe('Theme Toggle Tests', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:3000');
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(1000);
  });

  test('should have theme toggle button visible', async ({ page }) => {
    const themeToggle = page.locator('#theme-toggle');
    await expect(themeToggle).toBeVisible();
  });

  test('should start with dark theme and sun icon', async ({ page }) => {
    // Check initial data-theme attribute
    const dataTheme = await page.evaluate(() => {
      return document.documentElement.getAttribute('data-theme');
    });
    console.log('Initial data-theme:', dataTheme);
    
    // Check which icon is visible
    const sunIcon = page.locator('.theme-icon-light');
    const moonIcon = page.locator('.theme-icon-dark');
    
    const sunOpacity = await sunIcon.evaluate(el => window.getComputedStyle(el).opacity);
    const moonOpacity = await moonIcon.evaluate(el => window.getComputedStyle(el).opacity);
    
    console.log('Sun icon opacity:', sunOpacity);
    console.log('Moon icon opacity:', moonOpacity);
    
    // Should show sun in dark theme
    expect(sunOpacity).toBe('1');
    expect(moonOpacity).toBe('0');
  });

  test('should toggle theme and icon when clicked', async ({ page }) => {
    const themeToggle = page.locator('#theme-toggle');
    const sunIcon = page.locator('.theme-icon-light');
    const moonIcon = page.locator('.theme-icon-dark');
    
    // Get initial state
    const initialDataTheme = await page.evaluate(() => {
      return document.documentElement.getAttribute('data-theme');
    });
    console.log('Before click - data-theme:', initialDataTheme);
    
    const initialSunOpacity = await sunIcon.evaluate(el => window.getComputedStyle(el).opacity);
    const initialMoonOpacity = await moonIcon.evaluate(el => window.getComputedStyle(el).opacity);
    console.log('Before click - Sun opacity:', initialSunOpacity, 'Moon opacity:', initialMoonOpacity);
    
    // Click the toggle
    await themeToggle.click();
    await page.waitForTimeout(500); // Wait for transition
    
    // Get state after click
    const afterDataTheme = await page.evaluate(() => {
      return document.documentElement.getAttribute('data-theme');
    });
    console.log('After click - data-theme:', afterDataTheme);
    
    const afterSunOpacity = await sunIcon.evaluate(el => window.getComputedStyle(el).opacity);
    const afterMoonOpacity = await moonIcon.evaluate(el => window.getComputedStyle(el).opacity);
    console.log('After click - Sun opacity:', afterSunOpacity, 'Moon opacity:', afterMoonOpacity);
    
    // Theme should have changed
    expect(afterDataTheme).not.toBe(initialDataTheme);
    
    // Icons should have flipped
    expect(afterSunOpacity).not.toBe(initialSunOpacity);
    expect(afterMoonOpacity).not.toBe(initialMoonOpacity);
  });

  test('should change background color when theme toggles', async ({ page }) => {
    const body = page.locator('body');
    
    // Get initial background
    const initialBg = await body.evaluate(el => window.getComputedStyle(el).background);
    console.log('Initial background:', initialBg);
    
    // Click toggle
    const themeToggle = page.locator('#theme-toggle');
    await themeToggle.click();
    await page.waitForTimeout(500);
    
    // Get background after click
    const afterBg = await body.evaluate(el => window.getComputedStyle(el).background);
    console.log('After background:', afterBg);
    
    // Background should change
    expect(afterBg).not.toBe(initialBg);
  });

  test('should persist theme preference', async ({ page }) => {
    // Already starts in dark theme, click to go to light
    const themeToggle = page.locator('#theme-toggle');
    await themeToggle.click();
    await page.waitForTimeout(500);
    
    const lightTheme = await page.evaluate(() => {
      return document.documentElement.getAttribute('data-theme');
    });
    expect(lightTheme).toBe('light');
    
    // Reload page
    await page.reload();
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(1000);
    
    // Should still be light
    const persistedTheme = await page.evaluate(() => {
      return document.documentElement.getAttribute('data-theme');
    });
    expect(persistedTheme).toBe('light');
  });

  test('should handle multiple clicks correctly', async ({ page }) => {
    const themeToggle = page.locator('#theme-toggle');
    
    // Click multiple times
    for (let i = 0; i < 4; i++) {
      await themeToggle.click();
      await page.waitForTimeout(200);
      
      const currentTheme = await page.evaluate(() => {
        return document.documentElement.getAttribute('data-theme');
      });
      console.log(`Click ${i + 1} - Theme:`, currentTheme);
    }
    
    // Should end up back at dark theme (started dark, 4 clicks = even number)
    const finalTheme = await page.evaluate(() => {
      return document.documentElement.getAttribute('data-theme');
    });
    expect(finalTheme).toBe('dark');
  });
});