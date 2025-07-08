const { test, expect } = require('@playwright/test');

test.describe('JavaScript Redirect Debugging', () => {
  test('Debug window.location properties and redirect behavior', async ({ page }) => {
    // Navigate to knockknock page
    await page.goto('http://localhost:3001/knockknock');
    
    // Inject debugging script to capture all location properties
    const locationDebug = await page.evaluate(() => {
      return {
        href: window.location.href,
        protocol: window.location.protocol,
        host: window.location.host,
        hostname: window.location.hostname,
        port: window.location.port,
        pathname: window.location.pathname,
        origin: window.location.origin,
        search: window.location.search,
        hash: window.location.hash
      };
    });
    
    console.log('=== INITIAL LOCATION DEBUG ===');
    console.log(JSON.stringify(locationDebug, null, 2));
    
    // Test different redirect methods
    const redirectTests = [
      { method: 'absolute', url: '/admin' },
      { method: 'relative', url: './admin' },
      { method: 'origin', script: 'window.location.origin + "/admin"' },
      { method: 'constructed', script: 'window.location.protocol + "//" + window.location.host + "/admin"' }
    ];
    
    for (const testCase of redirectTests) {
      console.log(`\n=== TESTING ${testCase.method.toUpperCase()} REDIRECT ===`);
      
      // Navigate back to knockknock
      await page.goto('http://localhost:3001/knockknock');
      
      // Test the redirect and capture the target URL
      const targetUrl = await page.evaluate((test) => {
        let target;
        if (test.script) {
          target = eval(test.script);
        } else {
          target = test.url;
        }
        console.log(`${test.method} method target:`, target);
        return target;
      }, testCase);
      
      console.log(`Expected redirect URL: ${targetUrl}`);
      
      // Perform redirect
      await page.evaluate((url) => {
        window.location.href = url;
      }, targetUrl);
      
      // Wait for navigation and capture final URL
      await page.waitForTimeout(2000);
      const actualUrl = page.url();
      console.log(`Actual result URL: ${actualUrl}`);
      console.log(`Success: ${actualUrl.includes('admin') && !actualUrl.includes('knockknock')}`);
    }
  });
  
  test('Test login form redirect behavior', async ({ page }) => {
    await page.goto('http://localhost:3001/knockknock');
    
    // Capture console logs
    page.on('console', msg => console.log('PAGE LOG:', msg.text()));
    
    // Fill in login form
    await page.fill('#username', 'admin');
    await page.fill('#password', 'admin123');
    
    // Capture network requests
    const responses = [];
    page.on('response', response => {
      if (response.url().includes('login') || response.url().includes('admin')) {
        responses.push({
          url: response.url(),
          status: response.status(),
          headers: response.headers()
        });
      }
    });
    
    console.log('=== LOGIN FORM REDIRECT TEST ===');
    console.log('Current URL before login:', page.url());
    
    // Submit form and wait for redirect
    await page.click('button[type="submit"]');
    
    // Wait for the redirect delay (1.5 seconds) plus navigation time
    await page.waitForTimeout(4000);
    
    const finalUrl = page.url();
    console.log('Final URL after login:', finalUrl);
    console.log('Network responses:', responses);
    
    // Check if we ended up on admin page
    const isOnAdminPage = finalUrl.includes('/admin') && !finalUrl.includes('knockknock');
    console.log('Successfully reached admin page:', isOnAdminPage);
    
    // If not on admin page, check where we actually ended up
    if (!isOnAdminPage) {
      console.log('ERROR: Did not reach admin page!');
      console.log('Current page title:', await page.title());
      console.log('Current page content preview:', await page.textContent('body').then(text => text.substring(0, 200)));
    }
  });
  
  test('Manual redirect test with detailed logging', async ({ page }) => {
    await page.goto('http://localhost:3001/knockknock');
    
    // Inject detailed debugging
    await page.addInitScript(() => {
      window.debugRedirect = (method, target) => {
        console.log(`\n=== REDIRECT DEBUG: ${method} ===`);
        console.log('window.location.href:', window.location.href);
        console.log('window.location.origin:', window.location.origin);
        console.log('window.location.host:', window.location.host);
        console.log('window.location.port:', window.location.port);
        console.log('Target path:', target);
        
        let finalUrl;
        switch(method) {
          case 'origin':
            finalUrl = window.location.origin + target;
            break;
          case 'constructed': 
            finalUrl = window.location.protocol + '//' + window.location.host + target;
            break;
          case 'absolute':
            finalUrl = target;
            break;
          case 'relative':
            finalUrl = '.' + target;
            break;
        }
        
        console.log('Computed final URL:', finalUrl);
        return finalUrl;
      };
    });
    
    // Test each method manually
    const methods = ['origin', 'constructed', 'absolute', 'relative'];
    
    for (const method of methods) {
      await page.goto('http://localhost:3001/knockknock');
      
      const result = await page.evaluate((m) => {
        const url = window.debugRedirect(m, '/admin');
        window.location.href = url;
        return url;
      }, method);
      
      await page.waitForTimeout(2000);
      console.log(`\n${method.toUpperCase()} METHOD:`);
      console.log(`Target: ${result}`);
      console.log(`Actual: ${page.url()}`);
      console.log(`Success: ${page.url() === result}\n`);
    }
  });
});