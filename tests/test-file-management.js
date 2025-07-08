const { test, expect } = require('@playwright/test');
const fs = require('fs');
const path = require('path');

test.describe('File Management Tests', () => {
  let page;
  let adminToken;

  test.beforeAll(async ({ browser }) => {
    page = await browser.newPage();
    
    // Login to get admin token
    await page.goto('http://localhost:3000/knockknock/');
    await page.fill('#username', 'admin');
    await page.fill('#password', 'admin123');
    await page.click('button[type="submit"]');
    
    // Wait for redirect to admin page
    await page.waitForURL('**/admin/**', { timeout: 10000 });
    
    // Get token from localStorage
    adminToken = await page.evaluate(() => localStorage.getItem('adminToken'));
    console.log('Admin token obtained:', adminToken ? 'Yes' : 'No');
  });

  test.beforeEach(async () => {
    // Ensure we're on the admin page
    await page.goto('http://localhost:3000/admin/');
    await page.waitForLoadState('networkidle');
  });

  test('File Upload - Upload a single file', async () => {
    console.log('=== FILE UPLOAD TEST ===');
    
    // Create a test file
    const testFileContent = 'This is a test file for upload testing.';
    const testFileName = 'test-upload.txt';
    const testFilePath = path.join(__dirname, testFileName);
    fs.writeFileSync(testFilePath, testFileContent);
    
    console.log('Test file created:', testFilePath);
    
    // Monitor network requests
    const uploadRequests = [];
    page.on('request', request => {
      if (request.url().includes('/api/admin/files/upload')) {
        uploadRequests.push({
          url: request.url(),
          method: request.method(),
          headers: request.headers()
        });
        console.log('Upload request:', request.url(), request.method());
      }
    });
    
    const uploadResponses = [];
    page.on('response', response => {
      if (response.url().includes('/api/admin/files/upload')) {
        uploadResponses.push({
          url: response.url(),
          status: response.status(),
          statusText: response.statusText()
        });
        console.log('Upload response:', response.url(), response.status());
      }
    });
    
    // Click upload button
    await page.click('#uploadBtn');
    await page.waitForSelector('#uploadModal:not(.hidden)');
    console.log('Upload modal opened');
    
    // Upload file
    await page.setInputFiles('#fileInput', testFilePath);
    console.log('File selected for upload');
    
    // Submit upload
    await page.click('#uploadSubmitBtn');
    console.log('Upload submitted');
    
    // Wait for upload to complete (check for success message or modal close)
    try {
      await page.waitForSelector('.message.success', { timeout: 10000 });
      console.log('Success message appeared');
    } catch (error) {
      console.log('No success message, checking for error message');
      const errorMessage = await page.textContent('.message.error').catch(() => null);
      if (errorMessage) {
        console.log('Error message:', errorMessage);
      }
    }
    
    // Check if modal closed
    const modalHidden = await page.isHidden('#uploadModal');
    console.log('Modal hidden after upload:', modalHidden);
    
    // Log network activity
    console.log('Upload requests:', uploadRequests.length);
    console.log('Upload responses:', uploadResponses.length);
    
    if (uploadResponses.length > 0) {
      for (const response of uploadResponses) {
        console.log(`Response: ${response.status} ${response.statusText} - ${response.url}`);
      }
    }
    
    // Check if file appears in file list
    await page.waitForTimeout(2000); // Wait for file list to refresh
    const fileExists = await page.locator(`text="${testFileName}"`).count() > 0;
    console.log('File appears in list:', fileExists);
    
    // Clean up
    try {
      fs.unlinkSync(testFilePath);
    } catch (e) {
      console.log('Could not delete test file:', e.message);
    }
    
    // Assert that upload was successful
    expect(uploadResponses.length).toBeGreaterThan(0);
    if (uploadResponses.length > 0) {
      expect(uploadResponses[0].status).toBe(200);
    }
  });

  test('File Upload - Upload multiple files', async () => {
    console.log('=== MULTIPLE FILE UPLOAD TEST ===');
    
    // Create multiple test files
    const testFiles = [];
    for (let i = 1; i <= 3; i++) {
      const fileName = `test-multi-${i}.txt`;
      const filePath = path.join(__dirname, fileName);
      const content = `This is test file number ${i}`;
      fs.writeFileSync(filePath, content);
      testFiles.push(filePath);
    }
    
    console.log('Test files created:', testFiles.length);
    
    // Monitor upload requests
    const uploadResponses = [];
    page.on('response', response => {
      if (response.url().includes('/api/admin/files/upload')) {
        uploadResponses.push(response.status());
      }
    });
    
    // Open upload modal
    await page.click('#uploadBtn');
    await page.waitForSelector('#uploadModal:not(.hidden)');
    
    // Upload multiple files
    await page.setInputFiles('#fileInput', testFiles);
    await page.click('#uploadSubmitBtn');
    
    // Wait for upload completion
    await page.waitForTimeout(5000);
    
    // Clean up test files
    testFiles.forEach(filePath => {
      try {
        fs.unlinkSync(filePath);
      } catch (e) {
        console.log('Could not delete test file:', filePath);
      }
    });
    
    console.log('Upload responses received:', uploadResponses.length);
  });

  test('File Rename - Rename a file', async () => {
    console.log('=== FILE RENAME TEST ===');
    
    // First, create a test file to rename
    const originalName = 'test-rename-original.txt';
    const newName = 'test-rename-new.txt';
    const testContent = 'File for rename testing';
    const testFilePath = path.join(__dirname, originalName);
    fs.writeFileSync(testFilePath, testContent);
    
    // Upload the test file first
    await page.click('#uploadBtn');
    await page.waitForSelector('#uploadModal:not(.hidden)');
    await page.setInputFiles('#fileInput', testFilePath);
    await page.click('#uploadSubmitBtn');
    await page.waitForTimeout(3000);
    
    console.log('Test file uploaded for rename test');
    
    // Find the file in the list and click rename
    const fileRow = page.locator('.file-item').filter({ hasText: originalName });
    const renameButton = fileRow.locator('button:has-text("✏️")');
    
    // Monitor rename requests
    const renameRequests = [];
    const renameResponses = [];
    
    page.on('request', request => {
      if (request.url().includes('/api/admin/files/rename')) {
        renameRequests.push({
          url: request.url(),
          method: request.method(),
          postData: request.postDataJSON()
        });
        console.log('Rename request:', request.url());
        console.log('Rename data:', request.postDataJSON());
      }
    });
    
    page.on('response', response => {
      if (response.url().includes('/api/admin/files/rename')) {
        renameResponses.push({
          status: response.status(),
          url: response.url()
        });
        console.log('Rename response:', response.status());
      }
    });
    
    // Check if rename button exists
    const renameButtonExists = await renameButton.count() > 0;
    console.log('Rename button exists:', renameButtonExists);
    
    if (renameButtonExists) {
      // Click rename button
      await renameButton.click();
      console.log('Rename button clicked');
      
      // Wait for rename modal
      try {
        await page.waitForSelector('#renameModal:not(.hidden)', { timeout: 5000 });
        console.log('Rename modal opened');
        
        // Clear current name and enter new name
        await page.fill('#renameInput', '');
        await page.fill('#renameInput', newName);
        console.log('New name entered:', newName);
        
        // Submit rename
        await page.click('#renameSubmitBtn');
        console.log('Rename submitted');
        
        // Wait for response
        await page.waitForTimeout(3000);
        
        // Check if file was renamed
        const newFileExists = await page.locator(`text="${newName}"`).count() > 0;
        const oldFileExists = await page.locator(`text="${originalName}"`).count() > 0;
        
        console.log('New file exists in list:', newFileExists);
        console.log('Old file still exists:', oldFileExists);
        
      } catch (error) {
        console.log('Rename modal did not open:', error.message);
        
        // Check console errors
        const consoleLogs = [];
        page.on('console', msg => {
          if (msg.type() === 'error') {
            consoleLogs.push(msg.text());
          }
        });
        
        if (consoleLogs.length > 0) {
          console.log('Console errors:', consoleLogs);
        }
      }
    }
    
    // Log network activity
    console.log('Rename requests:', renameRequests.length);
    console.log('Rename responses:', renameResponses.length);
    
    // Clean up
    try {
      fs.unlinkSync(testFilePath);
    } catch (e) {
      console.log('Could not delete test file:', e.message);
    }
    
    // Assert rename functionality
    expect(renameButtonExists).toBe(true);
  });

  test('File Management UI - Check all buttons exist', async () => {
    console.log('=== UI BUTTONS TEST ===');
    
    // Check main action buttons
    const uploadBtn = await page.locator('#uploadBtn').count();
    const createFolderBtn = await page.locator('#createFolderBtn').count();
    const refreshBtn = await page.locator('#refreshBtn').count();
    
    console.log('Upload button exists:', uploadBtn > 0);
    console.log('Create folder button exists:', createFolderBtn > 0);
    console.log('Refresh button exists:', refreshBtn > 0);
    
    // Check if file list loads
    await page.waitForTimeout(2000);
    const fileListContent = await page.locator('#fileListContent').textContent();
    console.log('File list content loaded:', fileListContent.length > 0);
    
    // Check for any JavaScript errors in console
    const consoleErrors = [];
    page.on('console', msg => {
      if (msg.type() === 'error') {
        consoleErrors.push(msg.text());
      }
    });
    
    // Trigger some interactions to check for errors
    await page.click('#uploadBtn');
    await page.waitForTimeout(1000);
    await page.click('#cancelUploadBtn');
    
    await page.click('#createFolderBtn');
    await page.waitForTimeout(1000);
    await page.click('#cancelCreateFolderBtn');
    
    console.log('Console errors found:', consoleErrors.length);
    if (consoleErrors.length > 0) {
      console.log('Console errors:', consoleErrors);
    }
    
    expect(uploadBtn).toBeGreaterThan(0);
    expect(createFolderBtn).toBeGreaterThan(0);
    expect(refreshBtn).toBeGreaterThan(0);
  });

  test.afterAll(async () => {
    if (page) {
      await page.close();
    }
  });
});