const { test, expect } = require('@playwright/test');

test.describe('Browser Download Debug', () => {
  let page;
  let context;
  
  test.beforeAll(async ({ browser }) => {
    context = await browser.newContext();
    page = await context.newPage();
    
    // Capture console logs and errors
    page.on('console', msg => {
      console.log(`BROWSER ${msg.type().toUpperCase()}: ${msg.text()}`);
    });
    
    page.on('pageerror', error => {
      console.log(`PAGE ERROR: ${error.message}`);
    });
    
    // Login first
    await performLogin();
  });

  test.afterAll(async () => {
    await context.close();
  });

  async function performLogin() {
    await page.goto('http://localhost:3000/knockknock/');
    await page.waitForLoadState('networkidle');
    await page.fill('#username', 'admin');
    await page.fill('#password', 'admin');
    await page.click('button[type="submit"]');
    await page.waitForURL('**/admin/**', { timeout: 10000 });
  }

  test('should debug download function step by step', async () => {
    await page.goto('http://localhost:3000/admin/');
    await page.waitForLoadState('networkidle');
    
    // Create a test file
    await page.evaluate(async () => {
      const token = localStorage.getItem('adminToken');
      const response = await fetch('/api/admin/files/save/debug-download.txt', {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${token}`,
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ content: 'Debug download test content' })
      });
      const data = await response.json();
      console.log('File creation result:', data);
    });

    // Refresh to see the file
    await page.click('#refreshBtn');
    await page.waitForTimeout(2000);
    
    // Debug the download function step by step
    const debugResult = await page.evaluate(async () => {
      const filePath = 'debug-download.txt';
      const currentPath = '';
      const fullPath = currentPath ? `${currentPath}/${filePath}` : filePath;
      
      console.log('Starting download debug for:', fullPath);
      
      try {
        // Step 1: Get token
        const token = localStorage.getItem('adminToken');
        console.log('Token exists:', !!token);
        
        // Step 2: Make API request
        const response = await fetch(`/api/admin/files/download/${encodeURIComponent(fullPath)}`, {
          method: 'GET',
          headers: {
            'Authorization': `Bearer ${token}`
          }
        });
        
        console.log('Response status:', response.status);
        console.log('Response ok:', response.ok);
        console.log('Response headers:', Object.fromEntries(response.headers.entries()));
        
        if (!response.ok) {
          const errorText = await response.text();
          console.log('Error response:', errorText);
          return { error: `HTTP ${response.status}: ${errorText}` };
        }
        
        // Step 3: Check response content
        const contentType = response.headers.get('content-type');
        console.log('Content-Type:', contentType);
        
        // Step 4: Create blob
        const blob = await response.blob();
        console.log('Blob size:', blob.size);
        console.log('Blob type:', blob.type);
        
        // Step 5: Create download URL
        const url = window.URL.createObjectURL(blob);
        console.log('Blob URL created:', url);
        
        // Step 6: Create and click download link
        const fileName = filePath.split('/').pop();
        const a = document.createElement('a');
        a.href = url;
        a.download = fileName;
        a.style.display = 'none';
        
        console.log('Download link:', { href: a.href, download: a.download });
        
        document.body.appendChild(a);
        a.click();
        
        // Clean up
        document.body.removeChild(a);
        window.URL.revokeObjectURL(url);
        
        return { success: true, fileName, blobSize: blob.size };
        
      } catch (error) {
        console.error('Download error:', error);
        return { error: error.message };
      }
    });
    
    console.log('Debug result:', debugResult);
    
    // Check for success message
    await page.waitForTimeout(1000);
    
    // Clean up
    await page.evaluate(async () => {
      const token = localStorage.getItem('adminToken');
      await fetch('/api/admin/files/delete', {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${token}`,
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ path: 'debug-download.txt' })
      });
    });
    
    // Verify the debug was successful
    expect(debugResult.error).toBeUndefined();
    expect(debugResult.success).toBe(true);
  });

  test('should test clicking actual download button', async () => {
    await page.goto('http://localhost:3000/admin/');
    await page.waitForLoadState('networkidle');
    
    // Create test file
    await page.evaluate(async () => {
      const token = localStorage.getItem('adminToken');
      await fetch('/api/admin/files/save/click-test.txt', {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${token}`,
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ content: 'Click test content' })
      });
    });

    await page.click('#refreshBtn');
    await page.waitForTimeout(1000);
    
    // Find the file and click download
    const fileRow = page.locator('.file-item').filter({ has: page.locator('text=click-test.txt') });
    await expect(fileRow).toBeVisible();
    
    // Capture any download-related console logs
    const consoleMessages = [];
    page.on('console', msg => {
      consoleMessages.push(`${msg.type()}: ${msg.text()}`);
    });
    
    const downloadButton = fileRow.locator('.download-btn');
    await expect(downloadButton).toBeVisible();
    
    console.log('Clicking download button...');
    await downloadButton.click();
    
    // Wait for any messages
    await page.waitForTimeout(3000);
    
    console.log('Console messages after click:', consoleMessages);
    
    // Check if success message appeared
    const successMessage = page.locator('.message.success');
    const hasSuccessMessage = await successMessage.isVisible();
    console.log('Success message visible:', hasSuccessMessage);
    
    if (hasSuccessMessage) {
      const messageText = await successMessage.textContent();
      console.log('Success message text:', messageText);
    }
    
    // Clean up
    await page.evaluate(async () => {
      const token = localStorage.getItem('adminToken');
      await fetch('/api/admin/files/delete', {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${token}`,
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ path: 'click-test.txt' })
      });
    });
  });
});