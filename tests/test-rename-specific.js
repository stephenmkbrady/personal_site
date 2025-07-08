const { test, expect } = require('@playwright/test');
const fs = require('fs');
const path = require('path');

test.describe('Rename Function Debug Tests', () => {
  let page;

  test.beforeAll(async ({ browser }) => {
    page = await browser.newPage();
    
    // Login to get admin token
    await page.goto('http://localhost:3000/knockknock/');
    await page.fill('#username', 'admin');
    await page.fill('#password', 'admin123');
    await page.click('button[type="submit"]');
    
    // Wait for redirect to admin page
    await page.waitForURL('**/admin/**', { timeout: 10000 });
  });

  test('Debug rename modal functionality step by step', async () => {
    console.log('=== RENAME DEBUG TEST ===');
    
    // Navigate to admin page
    await page.goto('http://localhost:3000/admin/');
    await page.waitForLoadState('networkidle');
    
    // Create a test file for renaming
    const testFileName = 'test-rename-debug.txt';
    const testContent = 'File for rename debugging';
    const testFilePath = path.join(__dirname, testFileName);
    fs.writeFileSync(testFilePath, testContent);
    
    // Upload the test file
    console.log('Uploading test file...');
    await page.click('#uploadBtn');
    await page.waitForSelector('#uploadModal:not(.hidden)');
    await page.setInputFiles('#fileInput', testFilePath);
    await page.click('#uploadSubmitBtn');
    await page.waitForTimeout(3000);
    
    // Check if file appears in list
    const fileVisible = await page.locator(`text="${testFileName}"`).count() > 0;
    console.log('File visible in list:', fileVisible);
    
    if (fileVisible) {
      // Find the file row
      const fileRow = page.locator('.file-item').filter({ hasText: testFileName });
      const fileRowVisible = await fileRow.count() > 0;
      console.log('File row found:', fileRowVisible);
      
      if (fileRowVisible) {
        // Check all buttons in the file row
        const downloadBtn = fileRow.locator('button:has-text("⬇️")');
        const renameBtn = fileRow.locator('button:has-text("✏️")');
        const moveBtn = fileRow.locator('button:has-text("📁")');
        const deleteBtn = fileRow.locator('button:has-text("🗑️")');
        
        console.log('Download button exists:', await downloadBtn.count() > 0);
        console.log('Rename button exists:', await renameBtn.count() > 0);
        console.log('Move button exists:', await moveBtn.count() > 0);
        console.log('Delete button exists:', await deleteBtn.count() > 0);
        
        // Get the exact onclick handler
        const onclickHandler = await renameBtn.getAttribute('onclick');
        console.log('Rename button onclick:', onclickHandler);
        
        // Test clicking the rename button
        if (await renameBtn.count() > 0) {
          console.log('Clicking rename button...');
          
          // Listen for console errors
          const consoleErrors = [];
          page.on('console', msg => {
            if (msg.type() === 'error') {
              consoleErrors.push(msg.text());
            }
          });
          
          try {
            await renameBtn.click();
            console.log('Rename button clicked successfully');
            
            // Wait a bit and check if modal opened
            await page.waitForTimeout(1000);
            
            const modalVisible = await page.locator('#renameModal:not(.hidden)').count() > 0;
            console.log('Rename modal visible:', modalVisible);
            
            if (!modalVisible) {
              // Check for any JavaScript errors
              console.log('Console errors:', consoleErrors);
              
              // Try to manually call the function
              console.log('Trying to call showRenameModal manually...');
              await page.evaluate(() => {
                if (typeof showRenameModal === 'function') {
                  showRenameModal('test-file.txt', 'test-file.txt');
                  return 'Function called successfully';
                } else {
                  return 'Function not found';
                }
              });
              
              const modalVisibleAfterManual = await page.locator('#renameModal:not(.hidden)').count() > 0;
              console.log('Modal visible after manual call:', modalVisibleAfterManual);
            }
            
          } catch (error) {
            console.log('Error clicking rename button:', error.message);
          }
        }
      }
    }
    
    // Clean up
    try {
      fs.unlinkSync(testFilePath);
    } catch (e) {
      console.log('Could not delete test file:', e.message);
    }
  });

  test.afterAll(async () => {
    if (page) {
      await page.close();
    }
  });
});