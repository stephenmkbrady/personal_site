import { test, expect } from '@playwright/test';

test.describe('Card Display and Modal Tests', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:3003');
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);
  });

  test('should display cards with correct text and images when BlogStack is clicked', async ({ page }) => {
    // Click on blog CardStack
    await page.locator('.card-stack[data-category="blog"]').click();
    
    // Wait for cards to appear
    await page.waitForTimeout(3000);
    
    // Check that cards container exists
    await expect(page.locator('.cards-container[data-category="blog"]')).toBeVisible();
    
    // Check that cards are created
    const cards = page.locator('.cards-container[data-category="blog"] .card');
    await expect(cards).toHaveCount(3);
    
    // Check first card has correct title (not "Content Item")
    const firstCard = cards.first();
    const cardTitle = firstCard.locator('.card-text');
    await expect(cardTitle).toBeVisible();
    
    // Verify the title is not the default "Content Item"
    const titleText = await cardTitle.textContent();
    expect(titleText).not.toBe('Content Item');
    expect(titleText).toBeTruthy();
    console.log('First card title:', titleText);
    
    // Check that card has an image element
    const cardImage = firstCard.locator('.card-image');
    await expect(cardImage).toBeVisible();
    
    // Check that the image has a src attribute set
    const imageSrc = await cardImage.getAttribute('src');
    expect(imageSrc).toBeTruthy();
    expect(imageSrc).toContain('.jpg');
    console.log('Card image src:', imageSrc);
  });

  test('should display cards with correct image paths for blog content', async ({ page }) => {
    // Listen for failed image requests
    const failedRequests: string[] = [];
    page.on('requestfailed', request => {
      if (request.url().includes('.jpg') || request.url().includes('.png')) {
        failedRequests.push(request.url());
        console.log('Failed image request:', request.url());
      }
    });

    // Click on blog CardStack
    await page.locator('.card-stack[data-category="blog"]').click();
    await page.waitForTimeout(3000);

    // Get all cards
    const cards = page.locator('.cards-container[data-category="blog"] .card');
    const cardCount = await cards.count();
    
    for (let i = 0; i < cardCount; i++) {
      const card = cards.nth(i);
      const cardImage = card.locator('.card-image');
      
      if (await cardImage.isVisible()) {
        const src = await cardImage.getAttribute('src');
        console.log(`Card ${i} image src:`, src);
        
        // Check if it's trying to load placeholder.jpg (which should fail)
        if (src?.includes('placeholder.jpg') && !src.includes('/blog-placeholder.jpg')) {
          console.log(`Card ${i} is using incorrect image path: ${src}`);
        }
      }
    }

    // Check if any image requests failed
    await page.waitForTimeout(1000); // Wait for any pending requests
    console.log('Failed image requests:', failedRequests);
    
    // Expect no failed requests for blog-placeholder.jpg
    const blogPlaceholderFailures = failedRequests.filter(url => url.includes('blog-placeholder.jpg'));
    expect(blogPlaceholderFailures).toHaveLength(0);
  });

  test('should show correct content in modal when card is clicked', async ({ page }) => {
    // Wait for unified container to load automatically
    await page.waitForTimeout(4000);

    // Click on the first card (should be in unified container)
    const firstCard = page.locator('.unified-container .card').first();
    await firstCard.click();
    
    // Wait for modal to appear
    await page.waitForTimeout(1000);
    
    // Check that modal is visible
    const modal = page.locator('.modal');
    await expect(modal).toBeVisible();
    
    // Check that modal has content (not "undefined")
    const modalTitle = modal.locator('h1').first();
    await expect(modalTitle).toBeVisible();
    
    const titleText = await modalTitle.textContent();
    expect(titleText).not.toBe('undefined');
    expect(titleText).toBeTruthy();
    console.log('Modal title:', titleText);
    
    // Check that modal body has actual content (not "No content available")
    const modalContent = modal.locator('.markdown-content');
    const contentText = await modalContent.textContent();
    expect(contentText).not.toBe('No content available');
    expect(contentText).toBeTruthy();
    console.log('Modal content preview:', contentText?.substring(0, 100));
  });

  test('should create cards with data from API response', async ({ page }) => {
    const consoleMessages: string[] = [];
    page.on('console', msg => {
      if (msg.text().includes('Creating card') || msg.text().includes('Card items to create')) {
        consoleMessages.push(msg.text());
      }
    });

    // Click on blog CardStack
    await page.locator('.card-stack[data-category="blog"]').click();
    await page.waitForTimeout(3000);

    // Check console logs for card creation
    console.log('Card creation logs:', consoleMessages);
    
    // Verify cards were created with actual data
    const createLogs = consoleMessages.filter(msg => msg.includes('Creating card'));
    expect(createLogs.length).toBeGreaterThan(0);
    
    // Check that the logs show actual data, not default values
    const hasRealData = createLogs.some(log => 
      !log.includes('Content Item') && 
      !log.includes('/placeholder.jpg')
    );
    expect(hasRealData).toBe(true);
  });

  test('should handle API response data structure correctly', async ({ page }) => {
    // Intercept API calls to check response structure
    const apiResponses: any[] = [];
    
    page.route('**/api/content/blog', async route => {
      const response = await route.fetch();
      const data = await response.json();
      apiResponses.push(data);
      await route.fulfill({ response });
    });

    // Click on blog CardStack
    await page.locator('.card-stack[data-category="blog"]').click();
    await page.waitForTimeout(3000);

    // Check API response structure
    expect(apiResponses.length).toBe(1);
    const apiData = apiResponses[0];
    
    console.log('API Response structure:', JSON.stringify(apiData, null, 2));
    
    // Verify API response has expected structure
    expect(apiData.success).toBe(true);
    expect(apiData.data).toBeDefined();
    expect(Array.isArray(apiData.data)).toBe(true);
    expect(apiData.data.length).toBeGreaterThan(0);
    
    // Check first item structure
    const firstItem = apiData.data[0];
    expect(firstItem.metadata).toBeDefined();
    expect(firstItem.metadata.title).toBeDefined();
    expect(firstItem.slug).toBeDefined();
    
    console.log('First item metadata:', firstItem.metadata);
    console.log('First item image field:', firstItem.metadata.image);
  });

  test('should correctly load modal content for cards in unified container', async ({ page }) => {
    // Monitor API requests to verify correct category is used
    const apiRequests: string[] = [];
    page.on('request', request => {
      if (request.url().includes('/api/content/')) {
        apiRequests.push(request.url());
        console.log('API request:', request.url());
      }
    });

    // Wait for unified container to load automatically
    await page.waitForTimeout(4000);

    // Get cards from unified container
    const cards = page.locator('.unified-container .card');
    const cardCount = await cards.count();
    expect(cardCount).toBeGreaterThan(0);

    // Test modal for different card types
    for (let i = 0; i < Math.min(cardCount, 3); i++) {
      const card = cards.nth(i);
      
      // Get card category from data attributes
      const categoryAttr = await card.getAttribute('data-category');
      console.log(`Card ${i} category:`, categoryAttr);
      
      // Click the card
      await card.click();
      await page.waitForTimeout(1000);
      
      // Check modal appears
      const modal = page.locator('.modal');
      await expect(modal).toBeVisible();
      
      // Check content is loaded
      const modalContent = modal.locator('.markdown-content');
      const contentText = await modalContent.textContent();
      expect(contentText).not.toBe('No content available');
      
      // Close modal
      await modal.locator('.modal-close').click();
      await page.waitForTimeout(500);
    }

    // Verify no requests were made to '/api/content/unified/'
    const unifiedRequests = apiRequests.filter(url => url.includes('/unified/'));
    expect(unifiedRequests).toHaveLength(0);
    
    // Verify requests were made to correct category endpoints
    const validRequests = apiRequests.filter(url => 
      url.includes('/project/') || url.includes('/blog/') || url.includes('/github/')
    );
    expect(validRequests.length).toBeGreaterThan(0);
  });
});