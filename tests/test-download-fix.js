const { test, expect } = require('@playwright/test');

test.describe('Download Functionality Tests', () => {
  let page;
  let context;
  
  test.beforeAll(async ({ browser }) => {
    context = await browser.newContext();
    page = await context.newPage();
    
    // Login and get proper token
    await performLogin();
  });

  test.afterAll(async () => {
    await context.close();
  });

  async function performLogin() {
    console.log('Performing login...');
    
    // Go to login page
    await page.goto('http://localhost:3000/knockknock/');
    await page.waitForLoadState('networkidle');
    
    // Fill in the login form
    await page.fill('#username', 'admin');
    await page.fill('#password', 'admin');
    
    // Submit the form
    await page.click('button[type="submit"]');
    
    // Wait for the redirect
    await page.waitForURL('**/admin/**', { timeout: 10000 });
    
    console.log('Login successful, now on:', page.url());
  }

  test('should download files successfully with authentication', async () => {
    await page.goto('http://localhost:3000/admin/');
    await page.waitForLoadState('networkidle');
    
    // Create a test file to download
    const testContent = 'This is a test file for download functionality.\nLine 2\nLine 3';
    
    await page.evaluate(async (content) => {
      const token = localStorage.getItem('adminToken');
      const response = await fetch('/api/admin/files/save/download-test.txt', {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${token}`,
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ content })
      });
      const data = await response.json();
      if (!data.success) throw new Error(`Failed to create test file: ${data.message}`);
    }, testContent);

    // Refresh to see the new file
    await page.click('#refreshBtn');
    await page.waitForTimeout(1000);
    
    // Find the test file and click its download button
    const fileRow = page.locator('.file-item').filter({ has: page.locator('text=download-test.txt') });
    await expect(fileRow).toBeVisible();
    
    // Set up download listener
    const downloadPromise = page.waitForEvent('download');
    
    const downloadButton = fileRow.locator('.download-btn');
    await expect(downloadButton).toBeVisible();
    await downloadButton.click();
    
    // Wait for download to complete
    const download = await downloadPromise;
    
    // Verify download details
    expect(download.suggestedFilename()).toBe('download-test.txt');
    
    // Save to temporary location and verify content
    const downloadPath = await download.path();
    expect(downloadPath).toBeTruthy();
    
    // Wait for success message
    await expect(page.locator('.message.success')).toBeVisible({ timeout: 5000 });
    await expect(page.locator('.message.success')).toContainText('downloaded successfully');
    
    console.log('✓ File downloaded successfully');
    
    // Clean up - delete the test file
    await page.evaluate(async () => {
      const token = localStorage.getItem('adminToken');
      const response = await fetch('/api/admin/files/delete', {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${token}`,
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ path: 'download-test.txt' })
      });
      const data = await response.json();
      if (!data.success) console.warn(`Could not delete test file: ${data.message}`);
    });
  });

  test('should handle download errors gracefully', async () => {
    await page.goto('http://localhost:3000/admin/');
    await page.waitForLoadState('networkidle');
    
    // Try to download a non-existent file by calling the download function directly
    const result = await page.evaluate(async () => {
      // Call the downloadFile function with a non-existent file
      try {
        const token = localStorage.getItem('adminToken');
        const response = await fetch('/api/admin/files/download/non-existent-file.txt', {
          method: 'GET',
          headers: {
            'Authorization': `Bearer ${token}`
          }
        });
        
        return {
          status: response.status,
          ok: response.ok
        };
      } catch (error) {
        return { error: error.message };
      }
    });
    
    // Should return 404 for non-existent file
    expect(result.status).toBe(404);
    expect(result.ok).toBe(false);
    
    console.log('✓ Download error handling works correctly');
  });
});