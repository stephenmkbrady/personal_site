module.exports = {
  testDir: './tests',
  testMatch: ['**/*.test.js', 'test-*.js'],
  timeout: 60000,
  use: {
    headless: false,
    viewport: { width: 1280, height: 720 },
    screenshot: 'only-on-failure'
  }
};