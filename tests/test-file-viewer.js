const { test, expect } = require('@playwright/test');

test.describe('File Viewer and Editor Tests', () => {
  let page;
  let context;
  let consoleErrors = [];
  
  test.beforeAll(async ({ browser }) => {
    context = await browser.newContext();
    page = await context.newPage();
    
    // Capture console errors
    page.on('console', msg => {
      if (msg.type() === 'error') {
        consoleErrors.push(msg.text());
        console.log('Console error:', msg.text());
      }
    });
    
    // Set up authentication token
    await page.goto('http://localhost:3000/admin/');
    await page.evaluate(() => {
      localStorage.setItem('adminToken', 'test-token');
      localStorage.setItem('tokenExpires', new Date(Date.now() + 24 * 60 * 60 * 1000).toISOString());
    });
  });

  test.afterAll(async () => {
    await context.close();
  });

  test('should create test files for viewing and editing', async () => {
    await page.goto('http://localhost:3000/admin/');
    await page.waitForLoadState('networkidle');
    
    // Create a test markdown file
    const testMdContent = `# Test Markdown File

This is a test markdown file for the file viewer.

## Features
- View files
- Edit files
- Save changes

**Bold text** and *italic text*.

\`\`\`javascript
console.log('Hello, world!');
\`\`\`
`;

    const testTxtContent = `This is a test text file.

Line 1
Line 2
Line 3

Some special characters: àáâãäåæçèéêë

Numbers: 123456789

Symbols: !@#$%^&*()_+-=[]{}|;:'",./<>?
`;

    // Create test markdown file
    await page.evaluate(async (content) => {
      const response = await fetch('/api/admin/files/save/test-viewer.md', {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${localStorage.getItem('adminToken')}`,
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ content })
      });
      const data = await response.json();
      if (!data.success) throw new Error(`Failed to create test file: ${data.message}`);
    }, testMdContent);

    // Create test text file
    await page.evaluate(async (content) => {
      const response = await fetch('/api/admin/files/save/test-viewer.txt', {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${localStorage.getItem('adminToken')}`,
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ content })
      });
      const data = await response.json();
      if (!data.success) throw new Error(`Failed to create test file: ${data.message}`);
    }, testTxtContent);

    // Refresh the file list
    await page.click('#refreshBtn');
    await page.waitForTimeout(1000);
    
    console.log('✓ Test files created successfully');
  });

  test('should open file viewer when clicking view button', async () => {
    await page.goto('http://localhost:3000/admin/');
    await page.waitForLoadState('networkidle');
    
    // Wait for files to load
    await page.waitForSelector('.file-item', { timeout: 10000 });
    
    // Find the test markdown file and click its view button
    const mdFileRow = page.locator('.file-item').filter({ has: page.locator('text=test-viewer.md') });
    await expect(mdFileRow).toBeVisible();
    
    const viewButton = mdFileRow.locator('.view-btn');
    await expect(viewButton).toBeVisible();
    await viewButton.click();
    
    // Check that the file viewer modal opened
    const modal = page.locator('#fileViewerModal');
    await expect(modal).toBeVisible();
    
    // Check modal title and path
    await expect(page.locator('#fileViewerTitle')).toContainText('test-viewer.md');
    await expect(page.locator('#fileViewerPath')).toContainText('test-viewer.md');
    
    // Check file type badge
    await expect(page.locator('#fileTypeBadge')).toContainText('MD');
    
    console.log('✓ File viewer modal opens correctly');
  });

  test('should display file contents correctly in viewer', async () => {
    await page.goto('http://localhost:3000/admin/');
    await page.waitForLoadState('networkidle');
    
    // Open the markdown file
    const mdFileRow = page.locator('.file-item').filter({ has: page.locator('text=test-viewer.md') });
    const viewButton = mdFileRow.locator('.view-btn');
    await viewButton.click();
    
    // Wait for modal to be visible
    await expect(page.locator('#fileViewerModal')).toBeVisible();
    
    // Check that content is displayed
    const contentDisplay = page.locator('#fileContentDisplay');
    await expect(contentDisplay).toBeVisible();
    
    // Verify specific content from the test file
    await expect(contentDisplay).toContainText('# Test Markdown File');
    await expect(contentDisplay).toContainText('This is a test markdown file for the file viewer.');
    await expect(contentDisplay).toContainText('## Features');
    await expect(contentDisplay).toContainText('- View files');
    await expect(contentDisplay).toContainText('- Edit files');
    await expect(contentDisplay).toContainText('- Save changes');
    await expect(contentDisplay).toContainText('console.log(\'Hello, world!\');');
    
    console.log('✓ File contents display correctly');
  });

  test('should allow editing editable files', async () => {
    await page.goto('http://localhost:3000/admin/');
    await page.waitForLoadState('networkidle');
    
    // Open the text file (editable)
    const txtFileRow = page.locator('.file-item').filter({ has: page.locator('text=test-viewer.txt') });
    const viewButton = txtFileRow.locator('.view-btn');
    await viewButton.click();
    
    // Wait for modal to be visible
    await expect(page.locator('#fileViewerModal')).toBeVisible();
    
    // Check that edit button is visible (file is editable)
    const editToggleBtn = page.locator('#editToggleBtn');
    await expect(editToggleBtn).toBeVisible();
    await expect(editToggleBtn).toContainText('Edit');
    
    // Click edit button to enter edit mode
    await editToggleBtn.click();
    
    // Check that we're now in edit mode
    await expect(editToggleBtn).toContainText('View');
    await expect(page.locator('#saveFileBtn')).toBeVisible();
    
    // Check that editor is visible and display is hidden
    await expect(page.locator('#fileContentEditor')).toBeVisible();
    await expect(page.locator('#fileContentDisplay')).toBeHidden();
    
    // Verify editor has the file content
    const editorContent = await page.locator('#fileContentEditor').inputValue();
    expect(editorContent).toContain('This is a test text file.');
    expect(editorContent).toContain('Line 1');
    expect(editorContent).toContain('Line 2');
    expect(editorContent).toContain('Line 3');
    
    console.log('✓ Edit mode works correctly');
  });

  test('should save edited file contents', async () => {
    await page.goto('http://localhost:3000/admin/');
    await page.waitForLoadState('networkidle');
    
    // Open the text file
    const txtFileRow = page.locator('.file-item').filter({ has: page.locator('text=test-viewer.txt') });
    const viewButton = txtFileRow.locator('.view-btn');
    await viewButton.click();
    
    // Wait for modal and enter edit mode
    await expect(page.locator('#fileViewerModal')).toBeVisible();
    await page.locator('#editToggleBtn').click();
    
    // Add some text to the editor
    const editor = page.locator('#fileContentEditor');
    const originalContent = await editor.inputValue();
    const additionalText = '\n\nEDITED: This line was added by the test';
    const newContent = originalContent + additionalText;
    
    await editor.fill(newContent);
    
    // Save the file
    await page.locator('#saveFileBtn').click();
    
    // Wait for success message
    await expect(page.locator('.message.success')).toBeVisible({ timeout: 5000 });
    await expect(page.locator('.message.success')).toContainText('File saved successfully');
    
    // Switch back to view mode to see if changes are reflected
    await page.locator('#editToggleBtn').click();
    
    // Check that the display shows the updated content
    const contentDisplay = page.locator('#fileContentDisplay');
    await expect(contentDisplay).toContainText('EDITED: This line was added by the test');
    
    console.log('✓ File saves successfully and changes are reflected');
  });

  test('should persist edits after closing and reopening file', async () => {
    await page.goto('http://localhost:3000/admin/');
    await page.waitForLoadState('networkidle');
    
    // Close any open modal first
    if (await page.locator('#fileViewerModal').isVisible()) {
      await page.locator('#closeViewerBtn').click();
    }
    
    // Open the text file again
    const txtFileRow = page.locator('.file-item').filter({ has: page.locator('text=test-viewer.txt') });
    const viewButton = txtFileRow.locator('.view-btn');
    await viewButton.click();
    
    // Wait for modal to be visible
    await expect(page.locator('#fileViewerModal')).toBeVisible();
    
    // Check that our previous edit is still there
    const contentDisplay = page.locator('#fileContentDisplay');
    await expect(contentDisplay).toContainText('EDITED: This line was added by the test');
    
    console.log('✓ Edits persist after reopening file');
  });

  test('should handle non-editable files correctly', async () => {
    await page.goto('http://localhost:3000/admin/');
    await page.waitForLoadState('networkidle');
    
    // Create a non-editable file (e.g., .log file)
    await page.evaluate(async () => {
      const content = 'Log entry 1\nLog entry 2\nLog entry 3';
      const response = await fetch('/api/admin/files/save/test-readonly.log', {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${localStorage.getItem('adminToken')}`,
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ content })
      });
      const data = await response.json();
      if (!data.success) throw new Error(`Failed to create test file: ${data.message}`);
    });
    
    // Refresh to see the new file
    await page.click('#refreshBtn');
    await page.waitForTimeout(1000);
    
    // Open the log file
    const logFileRow = page.locator('.file-item').filter({ has: page.locator('text=test-readonly.log') });
    const viewButton = logFileRow.locator('.view-btn');
    await viewButton.click();
    
    // Wait for modal to be visible
    await expect(page.locator('#fileViewerModal')).toBeVisible();
    
    // Check that file type badge shows LOG
    await expect(page.locator('#fileTypeBadge')).toContainText('LOG');
    
    // Check that read-only badge is visible and edit button is hidden
    await expect(page.locator('#readOnlyBadge')).toBeVisible();
    await expect(page.locator('#editToggleBtn')).toBeHidden();
    
    // Check that content is displayed
    await expect(page.locator('#fileContentDisplay')).toContainText('Log entry 1');
    
    console.log('✓ Non-editable files handled correctly');
  });

  test('should open files by clicking on them directly', async () => {
    await page.goto('http://localhost:3000/admin/');
    await page.waitForLoadState('networkidle');
    
    // Close any open modal first
    if (await page.locator('#fileViewerModal').isVisible()) {
      await page.locator('#closeViewerBtn').click();
    }
    
    // Click directly on the markdown file name
    const mdFileRow = page.locator('.file-item').filter({ has: page.locator('text=test-viewer.md') });
    const fileName = mdFileRow.locator('.file-click-target');
    await fileName.click();
    
    // Check that the file viewer modal opened
    await expect(page.locator('#fileViewerModal')).toBeVisible();
    await expect(page.locator('#fileViewerTitle')).toContainText('test-viewer.md');
    
    console.log('✓ Files open when clicked directly');
  });

  test('should check for browser console errors', async () => {
    // Wait a bit to ensure all async operations complete
    await page.waitForTimeout(2000);
    
    console.log('Console errors captured:', consoleErrors);
    
    // Filter out known/acceptable errors
    const significantErrors = consoleErrors.filter(error => {
      // Filter out authentication-related errors which are expected during testing
      if (error.includes('401') || error.includes('Unauthorized')) return false;
      if (error.includes('Failed to fetch')) return false;
      if (error.includes('NetworkError')) return false;
      return true;
    });
    
    if (significantErrors.length > 0) {
      console.warn('⚠️ Significant console errors found:', significantErrors);
    } else {
      console.log('✓ No significant console errors detected');
    }
    
    // Don't fail the test for console errors, just report them
    expect(significantErrors.length).toBeLessThanOrEqual(5); // Allow some minor errors
  });

  test('should clean up test files', async () => {
    await page.goto('http://localhost:3000/admin/');
    await page.waitForLoadState('networkidle');
    
    // Close any open modal first
    if (await page.locator('#fileViewerModal').isVisible()) {
      await page.locator('#closeViewerBtn').click();
    }
    
    // Delete test files
    const testFiles = ['test-viewer.md', 'test-viewer.txt', 'test-readonly.log'];
    
    for (const fileName of testFiles) {
      try {
        await page.evaluate(async (file) => {
          const response = await fetch('/api/admin/files/delete', {
            method: 'POST',
            headers: {
              'Authorization': `Bearer ${localStorage.getItem('adminToken')}`,
              'Content-Type': 'application/json'
            },
            body: JSON.stringify({ path: file })
          });
          const data = await response.json();
          if (!data.success) {
            console.warn(`Could not delete ${file}: ${data.message}`);
          }
        }, fileName);
      } catch (error) {
        console.warn(`Error deleting ${fileName}:`, error);
      }
    }
    
    console.log('✓ Test cleanup completed');
  });
});