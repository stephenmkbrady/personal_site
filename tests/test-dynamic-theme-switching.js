const { test, expect } = require('@playwright/test');

test.describe('Dynamic Theme Switching', () => {
  test('should update card images when theme changes', async ({ page }) => {
    // Visit the portfolio site
    await page.goto('http://localhost:3000');
    await page.waitForLoadState('networkidle');
    
    // Wait for cards to load
    await page.waitForSelector('.card', { timeout: 10000 });
    
    console.log('Testing dynamic theme switching...');
    
    // Get initial theme and card images
    const initialState = await page.evaluate(() => {
      const theme = document.documentElement.getAttribute('data-theme');
      const cards = document.querySelectorAll('.card');
      const cardImages = [];
      
      cards.forEach((card, index) => {
        const img = card.querySelector('.card-image');
        const cardData = card.getAttribute('data-full-content');
        
        if (img && cardData) {
          try {
            const data = JSON.parse(cardData);
            
            // Check if this card should use theme-aware placeholder
            let shouldUseThemePlaceholder = false;
            
            if (data.category === 'github') {
              shouldUseThemePlaceholder = !data.image;
            } else {
              shouldUseThemePlaceholder = !data.metadata?.image && !data.image;
            }
            
            if (shouldUseThemePlaceholder) {
              cardImages.push({
                index,
                src: img.src,
                alt: img.alt,
                shouldUseThemePlaceholder: true
              });
            }
          } catch (error) {
            console.error('Error parsing card data:', error);
          }
        }
      });
      
      return {
        theme,
        cardImages
      };
    });
    
    console.log('Initial state:', initialState);
    
    // Only proceed if we have cards that should use theme-aware placeholders
    if (initialState.cardImages.length === 0) {
      console.log('No cards using theme-aware placeholders found, skipping test');
      return;
    }
    
    // Toggle theme
    const themeToggle = page.locator('#theme-toggle');
    await themeToggle.click();
    
    // Wait for theme change and mutation observer to update images
    await page.waitForTimeout(1000);
    
    // Get new state after theme change
    const newState = await page.evaluate(() => {
      const theme = document.documentElement.getAttribute('data-theme');
      const cards = document.querySelectorAll('.card');
      const cardImages = [];
      
      cards.forEach((card, index) => {
        const img = card.querySelector('.card-image');
        const cardData = card.getAttribute('data-full-content');
        
        if (img && cardData) {
          try {
            const data = JSON.parse(cardData);
            
            // Check if this card should use theme-aware placeholder
            let shouldUseThemePlaceholder = false;
            
            if (data.category === 'github') {
              shouldUseThemePlaceholder = !data.image;
            } else {
              shouldUseThemePlaceholder = !data.metadata?.image && !data.image;
            }
            
            if (shouldUseThemePlaceholder) {
              cardImages.push({
                index,
                src: img.src,
                alt: img.alt,
                shouldUseThemePlaceholder: true
              });
            }
          } catch (error) {
            console.error('Error parsing card data:', error);
          }
        }
      });
      
      return {
        theme,
        cardImages
      };
    });
    
    console.log('New state:', newState);
    
    // Verify theme changed
    expect(newState.theme).not.toBe(initialState.theme);
    
    // Verify card images changed for theme-aware placeholders
    expect(newState.cardImages.length).toBe(initialState.cardImages.length);
    
    for (let i = 0; i < initialState.cardImages.length; i++) {
      const initialCard = initialState.cardImages[i];
      const newCard = newState.cardImages[i];
      
      console.log(`Card ${i}: ${initialCard.src} → ${newCard.src}`);
      
      // Images should have changed
      expect(newCard.src).not.toBe(initialCard.src);
      
      // Should use correct theme-aware placeholder
      if (newState.theme === 'light') {
        expect(newCard.src).toContain('/light/card.jpg');
      } else {
        expect(newCard.src).toContain('/dark/card.jpg');
      }
    }
    
    console.log('Dynamic theme switching test completed successfully!');
  });
  
  test('should handle multiple theme toggles', async ({ page }) => {
    // Visit the portfolio site
    await page.goto('http://localhost:3000');
    await page.waitForLoadState('networkidle');
    
    // Wait for cards to load
    await page.waitForSelector('.card', { timeout: 10000 });
    
    const themeToggle = page.locator('#theme-toggle');
    
    // Test multiple theme toggles
    for (let i = 0; i < 3; i++) {
      console.log(`Theme toggle ${i + 1}...`);
      
      const beforeTheme = await page.evaluate(() => {
        return document.documentElement.getAttribute('data-theme');
      });
      
      await themeToggle.click();
      await page.waitForTimeout(500);
      
      const afterTheme = await page.evaluate(() => {
        return document.documentElement.getAttribute('data-theme');
      });
      
      console.log(`Toggle ${i + 1}: ${beforeTheme} → ${afterTheme}`);
      
      // Theme should have changed
      expect(afterTheme).not.toBe(beforeTheme);
    }
    
    console.log('Multiple theme toggles test completed successfully!');
  });
});