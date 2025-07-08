module.exports = {
  testDir: './tests',
  testMatch: ['test-redirect.js', 'test-file-management.js', 'test-modal-debug.js', 'test-rename-specific.js'],
  timeout: 60000,
  use: {
    headless: false,
    viewport: { width: 1280, height: 720 },
    screenshot: 'only-on-failure'
  }
};