const { test, expect } = require('@playwright/test');

test.describe('Debug Theme Switching', () => {
  test('should debug theme switching mechanism', async ({ page }) => {
    // Capture console logs
    const consoleMessages = [];
    page.on('console', msg => {
      consoleMessages.push(`${msg.type()}: ${msg.text()}`);
    });
    
    // Visit the portfolio site
    await page.goto('http://localhost:3000');
    await page.waitForLoadState('networkidle');
    
    // Wait for cards to load
    await page.waitForSelector('.card', { timeout: 10000 });
    
    console.log('Initial console messages:', consoleMessages);
    
    // Test the mutation observer directly
    const debugResult = await page.evaluate(() => {
      // Check if mutation observer is set up
      const hasObserver = window.cardThemeObserver !== undefined;
      
      // Test manual theme change
      const initialTheme = document.documentElement.getAttribute('data-theme');
      
      // Test getThemeAwarePlaceholder function
      function getThemeAwarePlaceholder() {
        const isLightTheme = document.documentElement.getAttribute('data-theme') === 'light';
        return isLightTheme ? '/light/card.jpg' : '/dark/card.jpg';
      }
      
      const initialPlaceholder = getThemeAwarePlaceholder();
      
      // Manually trigger theme change
      document.documentElement.setAttribute('data-theme', 'light');
      const newPlaceholder = getThemeAwarePlaceholder();
      
      // Check if updateCardImagesForTheme function exists
      const hasUpdateFunction = typeof window.updateCardImagesForTheme === 'function';
      
      return {
        hasObserver,
        hasUpdateFunction,
        initialTheme,
        initialPlaceholder,
        newPlaceholder,
        functions: Object.keys(window).filter(key => key.includes('card') || key.includes('theme'))
      };
    });
    
    console.log('Debug result:', debugResult);
    
    // Toggle theme using the button
    const themeToggle = page.locator('#theme-toggle');
    await themeToggle.click();
    await page.waitForTimeout(2000);
    
    console.log('Console messages after toggle:', consoleMessages);
    
    // Check final state
    const finalState = await page.evaluate(() => {
      const theme = document.documentElement.getAttribute('data-theme');
      const sampleCard = document.querySelector('.card');
      const sampleImage = sampleCard ? sampleCard.querySelector('.card-image') : null;
      
      return {
        theme,
        sampleImageSrc: sampleImage ? sampleImage.src : null
      };
    });
    
    console.log('Final state:', finalState);
    
    // Log all console messages
    console.log('All console messages:', consoleMessages);
  });
});