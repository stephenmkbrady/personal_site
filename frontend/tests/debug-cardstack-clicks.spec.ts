import { test, expect } from '@playwright/test';

test.describe('Debug CardStack Click Behavior', () => {
  test('should capture and analyze all console logs during CardStack interaction', async ({ page }) => {
    const allConsoleMessages: Array<{
      type: string;
      text: string;
      timestamp: number;
    }> = [];

    // Capture all console messages with timestamps
    page.on('console', msg => {
      allConsoleMessages.push({
        type: msg.type(),
        text: msg.text(),
        timestamp: Date.now()
      });
    });

    // Navigate to the page
    await page.goto('http://localhost:3003');
    await page.waitForLoadState('networkidle');

    // Wait a bit for initialization
    await page.waitForTimeout(2000);

    console.log('\n=== INITIALIZATION LOGS ===');
    const initLogs = allConsoleMessages.filter(msg => 
      msg.text.includes('CardInteractions') || 
      msg.text.includes('Found') || 
      msg.text.includes('Initializing')
    );
    initLogs.forEach(log => console.log(`[${log.type}] ${log.text}`));

    // Clear previous logs for cleaner CardStack click analysis
    allConsoleMessages.length = 0;

    // Click on project CardStack and capture detailed logs
    console.log('\n=== CLICKING PROJECT CARDSTACK ===');
    await page.locator('.card-stack[data-category="project"]').click();

    // Wait for all async operations to complete
    await page.waitForTimeout(5000);

    // Analyze logs in order
    console.log('\n=== ALL LOGS AFTER PROJECT CLICK ===');
    allConsoleMessages.forEach((log, index) => {
      console.log(`${index + 1}. [${log.type}] ${log.text}`);
    });

    // Test specific log patterns
    const clickLogs = allConsoleMessages.filter(msg => msg.text.includes('CardStack clicked'));
    const handleLogs = allConsoleMessages.filter(msg => msg.text.includes('handleCardStackClick called'));
    const domLogs = allConsoleMessages.filter(msg => msg.text.includes('DOM ready') || msg.text.includes('Grid element'));
    const apiLogs = allConsoleMessages.filter(msg => msg.text.includes('Fetching from API') || msg.text.includes('API response'));
    const cardLogs = allConsoleMessages.filter(msg => msg.text.includes('Card items to create') || msg.text.includes('Creating card'));

    console.log('\n=== LOG ANALYSIS ===');
    console.log(`Click logs: ${clickLogs.length}`);
    console.log(`Handle logs: ${handleLogs.length}`);
    console.log(`DOM logs: ${domLogs.length}`);
    console.log(`API logs: ${apiLogs.length}`);
    console.log(`Card creation logs: ${cardLogs.length}`);

    // Assert we have the expected logs
    expect(clickLogs.length).toBeGreaterThan(0);
    expect(handleLogs.length).toBeGreaterThan(0);
    expect(domLogs.length).toBeGreaterThan(0);

    // Check for API call
    if (apiLogs.length > 0) {
      console.log('\n=== API LOGS DETAILS ===');
      apiLogs.forEach(log => console.log(`- ${log.text}`));
    } else {
      console.log('\n⚠️  NO API LOGS FOUND - API call may have failed');
    }

    // Check for card creation
    if (cardLogs.length > 0) {
      console.log('\n=== CARD CREATION LOGS ===');
      cardLogs.forEach(log => console.log(`- ${log.text}`));
    } else {
      console.log('\n⚠️  NO CARD CREATION LOGS - Cards may not be created');
    }

    // Check if cards container exists in DOM
    const cardsContainer = page.locator('.cards-container[data-category="project"]');
    const containerExists = await cardsContainer.count() > 0;
    console.log(`\n=== DOM STATE ===`);
    console.log(`Cards container exists: ${containerExists}`);

    if (containerExists) {
      const containerHTML = await cardsContainer.innerHTML();
      console.log(`Container content: ${containerHTML.substring(0, 200)}...`);
      
      const cardCount = await cardsContainer.locator('.card').count();
      console.log(`Cards in container: ${cardCount}`);
    }

    // Check if CardStack has 'active' class
    const stackElement = page.locator('.card-stack[data-category="project"]');
    const hasActiveClass = await stackElement.evaluate(el => el.classList.contains('active'));
    console.log(`CardStack has active class: ${hasActiveClass}`);
  });

  test('should test API connectivity independently', async ({ page }) => {
    await page.goto('http://localhost:3003');

    // Test backend connectivity
    const backendTest = await page.evaluate(async () => {
      try {
        const healthResponse = await fetch('http://localhost:4000/api/health');
        const healthData = await healthResponse.json();
        
        const contentResponse = await fetch('http://localhost:4000/api/content/project');
        const contentData = await contentResponse.json();
        
        return {
          health: { status: healthResponse.status, data: healthData },
          content: { status: contentResponse.status, data: contentData }
        };
      } catch (error) {
        return { error: error.message };
      }
    });

    console.log('\n=== BACKEND CONNECTIVITY TEST ===');
    console.log(JSON.stringify(backendTest, null, 2));

    expect(backendTest.health?.status).toBe(200);
    expect(backendTest.content?.status).toBe(200);
  });

  test('should manually trigger card creation functions', async ({ page }) => {
    const consoleMessages: string[] = [];
    page.on('console', msg => consoleMessages.push(msg.text()));

    await page.goto('http://localhost:3003');
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    // Test if we can manually call the functions
    const manualTest = await page.evaluate(async () => {
      // Check if functions exist on window or are accessible
      const grid = document.getElementById('grid');
      const cardStacks = document.querySelectorAll('.card-stack');
      const projectStack = document.querySelector('.card-stack[data-category="project"]');

      return {
        gridExists: !!grid,
        cardStackCount: cardStacks.length,
        projectStackExists: !!projectStack,
        projectStackCategory: projectStack?.getAttribute('data-category'),
        activeCardStacks: window.activeCardStacks ? window.activeCardStacks.size : 'not accessible'
      };
    });

    console.log('\n=== MANUAL FUNCTION TEST ===');
    console.log(JSON.stringify(manualTest, null, 2));

    // Try to manually trigger the click handler
    await page.evaluate(() => {
      const projectStack = document.querySelector('.card-stack[data-category="project"]') as HTMLElement;
      if (projectStack) {
        // Dispatch a manual click event
        const clickEvent = new MouseEvent('click', {
          bubbles: true,
          cancelable: true,
          view: window
        });
        projectStack.dispatchEvent(clickEvent);
      }
    });

    await page.waitForTimeout(3000);

    console.log('\n=== CONSOLE LOGS FROM MANUAL TRIGGER ===');
    consoleMessages.forEach((msg, index) => {
      console.log(`${index + 1}. ${msg}`);
    });
  });

  test('should inspect DOM structure after click', async ({ page }) => {
    await page.goto('http://localhost:3003');
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    // Get initial DOM state
    const initialState = await page.evaluate(() => {
      return {
        gridChildren: document.getElementById('grid')?.children.length || 0,
        bodyChildren: document.body.children.length,
        cardContainers: document.querySelectorAll('.cards-container').length
      };
    });

    console.log('\n=== INITIAL DOM STATE ===');
    console.log(JSON.stringify(initialState, null, 2));

    // Click and wait
    await page.locator('.card-stack[data-category="blog"]').click();
    await page.waitForTimeout(4000);

    // Get final DOM state
    const finalState = await page.evaluate(() => {
      const grid = document.getElementById('grid');
      const cardsContainers = document.querySelectorAll('.cards-container');
      
      return {
        gridChildren: grid?.children.length || 0,
        bodyChildren: document.body.children.length,
        cardContainers: cardsContainers.length,
        containerDetails: Array.from(cardsContainers).map(container => ({
          category: container.getAttribute('data-category'),
          childCount: container.children.length,
          innerHTML: container.innerHTML.substring(0, 100)
        }))
      };
    });

    console.log('\n=== FINAL DOM STATE ===');
    console.log(JSON.stringify(finalState, null, 2));

    // Compare states
    console.log('\n=== DOM CHANGES ===');
    console.log(`Grid children: ${initialState.gridChildren} → ${finalState.gridChildren}`);
    console.log(`Body children: ${initialState.bodyChildren} → ${finalState.bodyChildren}`);
    console.log(`Card containers: ${initialState.cardContainers} → ${finalState.cardContainers}`);
  });
});