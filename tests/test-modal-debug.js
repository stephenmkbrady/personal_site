const { test, expect } = require('@playwright/test');

test('Debug modal structure and functions', async ({ page }) => {
  console.log('=== MODAL DEBUG TEST ===');
  
  // Login
  await page.goto('http://localhost:3000/knockknock/');
  await page.fill('#username', 'admin');
  await page.fill('#password', 'admin123');
  await page.click('button[type="submit"]');
  await page.waitForURL('**/admin/**', { timeout: 10000 });
  
  // Go to admin page
  await page.goto('http://localhost:3000/admin/');
  await page.waitForLoadState('networkidle');
  
  // Check if modal elements exist
  const uploadModal = await page.locator('#uploadModal').count();
  const renameModal = await page.locator('#renameModal').count();
  const createFolderModal = await page.locator('#createFolderModal').count();
  const moveModal = await page.locator('#moveModal').count();
  const confirmModal = await page.locator('#confirmModal').count();
  
  console.log('Upload modal exists:', uploadModal > 0);
  console.log('Rename modal exists:', renameModal > 0);
  console.log('Create folder modal exists:', createFolderModal > 0);
  console.log('Move modal exists:', moveModal > 0);
  console.log('Confirm modal exists:', confirmModal > 0);
  
  // Check modal classes
  if (renameModal > 0) {
    const modalClasses = await page.locator('#renameModal').getAttribute('class');
    console.log('Rename modal classes:', modalClasses);
    
    const renameInput = await page.locator('#renameInput').count();
    const renameSubmitBtn = await page.locator('#renameSubmitBtn').count();
    const cancelRenameBtn = await page.locator('#cancelRenameBtn').count();
    
    console.log('Rename input exists:', renameInput > 0);
    console.log('Rename submit button exists:', renameSubmitBtn > 0);
    console.log('Cancel rename button exists:', cancelRenameBtn > 0);
  }
  
  // Test the functions directly
  const functionTests = await page.evaluate(() => {
    const results = {};
    
    // Test if functions exist
    results.showModalExists = typeof showModal === 'function';
    results.hideModalExists = typeof hideModal === 'function';
    results.showRenameModalExists = typeof showRenameModal === 'function';
    
    // Test showModal function
    if (typeof showModal === 'function') {
      try {
        showModal('renameModal');
        results.showModalCalled = true;
        
        // Check if modal is visible after calling showModal
        const modal = document.getElementById('renameModal');
        if (modal) {
          results.modalClassesAfterShow = modal.className;
          results.modalVisibleAfterShow = !modal.classList.contains('hidden');
        }
      } catch (error) {
        results.showModalError = error.message;
      }
    }
    
    return results;
  });
  
  console.log('Function test results:', functionTests);
  
  // Check if modal is now visible
  const modalVisibleAfterEval = await page.locator('#renameModal:not(.hidden)').count() > 0;
  console.log('Modal visible after evaluation:', modalVisibleAfterEval);
  
  // Test upload modal (which we know works)
  console.log('Testing upload modal (working one)...');
  await page.click('#uploadBtn');
  await page.waitForTimeout(1000);
  
  const uploadModalVisible = await page.locator('#uploadModal:not(.hidden)').count() > 0;
  console.log('Upload modal visible after click:', uploadModalVisible);
  
  // Cancel upload modal
  if (uploadModalVisible) {
    await page.click('#cancelUploadBtn');
  }
});