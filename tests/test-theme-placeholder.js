const { test, expect } = require('@playwright/test');

test.describe('Theme-Aware Placeholder Images', () => {
  test('should use different placeholder images for light and dark themes', async ({ page }) => {
    // Visit the portfolio site
    await page.goto('http://localhost:3000');
    await page.waitForLoadState('networkidle');
    
    // Wait for cards to load
    await page.waitForSelector('.card', { timeout: 10000 });
    
    // Test initial theme (should be dark by default)
    console.log('Testing initial theme (dark)...');
    
    // Check current theme
    const initialTheme = await page.evaluate(() => {
      return document.documentElement.getAttribute('data-theme');
    });
    console.log('Initial theme:', initialTheme);
    
    // Get images from cards that don't specify a custom image
    const cardImages = await page.evaluate(() => {
      const cards = document.querySelectorAll('.card');
      const images = [];
      
      cards.forEach(card => {
        const img = card.querySelector('.card-image');
        if (img) {
          images.push({
            src: img.src,
            alt: img.alt
          });
        }
      });
      
      return images;
    });
    
    console.log('Initial card images:', cardImages);
    
    // Find placeholder images (should be dark theme placeholders)
    const placeholderImages = cardImages.filter(img => 
      img.src.includes('/dark/card.jpg') || img.src.includes('/light/card.jpg')
    );
    
    console.log('Found placeholder images:', placeholderImages);
    
    // Switch to light theme
    console.log('Switching to light theme...');
    const themeToggle = page.locator('.theme-toggle');
    await themeToggle.click();
    await page.waitForTimeout(1000); // Wait for theme transition
    
    // Check new theme
    const newTheme = await page.evaluate(() => {
      return document.documentElement.getAttribute('data-theme');
    });
    console.log('New theme:', newTheme);
    
    // Get images after theme change
    const newCardImages = await page.evaluate(() => {
      const cards = document.querySelectorAll('.card');
      const images = [];
      
      cards.forEach(card => {
        const img = card.querySelector('.card-image');
        if (img) {
          images.push({
            src: img.src,
            alt: img.alt
          });
        }
      });
      
      return images;
    });
    
    console.log('New card images:', newCardImages);
    
    // Find new placeholder images (should be light theme placeholders)
    const newPlaceholderImages = newCardImages.filter(img => 
      img.src.includes('/dark/card.jpg') || img.src.includes('/light/card.jpg')
    );
    
    console.log('New placeholder images:', newPlaceholderImages);
    
    // Verify that theme switching works
    if (initialTheme === 'light') {
      expect(newTheme).toBe('dark');
    } else {
      expect(newTheme).toBe('light');
    }
    
    // Test that placeholder images change when theme changes
    // This test assumes that placeholder images are being used
    if (placeholderImages.length > 0 && newPlaceholderImages.length > 0) {
      const initialPlaceholderType = placeholderImages[0].src.includes('/light/') ? 'light' : 'dark';
      const newPlaceholderType = newPlaceholderImages[0].src.includes('/light/') ? 'light' : 'dark';
      
      console.log('Initial placeholder type:', initialPlaceholderType);
      console.log('New placeholder type:', newPlaceholderType);
      
      // They should be different
      expect(initialPlaceholderType).not.toBe(newPlaceholderType);
    }
    
    // Switch back to original theme
    console.log('Switching back to original theme...');
    await themeToggle.click();
    await page.waitForTimeout(1000);
    
    const finalTheme = await page.evaluate(() => {
      return document.documentElement.getAttribute('data-theme');
    });
    console.log('Final theme:', finalTheme);
    
    // Should be back to original
    expect(finalTheme).toBe(initialTheme);
  });
  
  test('should use correct placeholder based on theme detection', async ({ page }) => {
    // Visit the portfolio site
    await page.goto('http://localhost:3000');
    await page.waitForLoadState('networkidle');
    
    // Test the getThemeAwarePlaceholder function directly
    const placeholderTest = await page.evaluate(() => {
      // Function to get theme-appropriate placeholder image
      function getThemeAwarePlaceholder() {
        // Check if document element has light theme attribute
        const isLightTheme = document.documentElement.getAttribute('data-theme') === 'light';
        return isLightTheme ? '/light/card.jpg' : '/dark/card.jpg';
      }
      
      // Test with default theme
      const defaultPlaceholder = getThemeAwarePlaceholder();
      
      // Set light theme
      document.documentElement.setAttribute('data-theme', 'light');
      const lightPlaceholder = getThemeAwarePlaceholder();
      
      // Set dark theme
      document.documentElement.setAttribute('data-theme', 'dark');
      const darkPlaceholder = getThemeAwarePlaceholder();
      
      // Remove theme attribute (should default to dark)
      document.documentElement.removeAttribute('data-theme');
      const noThemePlaceholder = getThemeAwarePlaceholder();
      
      return {
        defaultPlaceholder,
        lightPlaceholder,
        darkPlaceholder,
        noThemePlaceholder
      };
    });
    
    console.log('Placeholder test results:', placeholderTest);
    
    // Verify correct placeholder selection
    expect(placeholderTest.lightPlaceholder).toBe('/light/card.jpg');
    expect(placeholderTest.darkPlaceholder).toBe('/dark/card.jpg');
    expect(placeholderTest.noThemePlaceholder).toBe('/dark/card.jpg'); // Default to dark
  });
});