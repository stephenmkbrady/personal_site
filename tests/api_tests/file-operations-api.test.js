const { test, expect } = require('@playwright/test');

test.describe('File Operations API Tests', () => {
  let authToken = null;
  let baseURL = 'http://localhost:3000';
  
  test.beforeAll(async ({ request }) => {
    // Get authentication token before running tests
    console.log('🔐 Getting authentication token...');
    
    const loginResponse = await request.post(`${baseURL}/api/auth/login`, {
      data: {
        username: 'admin',
        password: 'admin'
      }
    });
    
    expect(loginResponse.ok()).toBeTruthy();
    const loginData = await loginResponse.json();
    expect(loginData.success).toBe(true);
    
    authToken = loginData.data.token;
    console.log('✓ Authentication token obtained');
  });

  test.describe('Authentication API', () => {
    test('should login with valid credentials', async ({ request }) => {
      const response = await request.post(`${baseURL}/api/auth/login`, {
        data: {
          username: 'admin',
          password: 'admin'
        }
      });
      
      expect(response.ok()).toBeTruthy();
      const data = await response.json();
      
      expect(data.success).toBe(true);
      expect(data.data.token).toBeDefined();
      expect(data.data.expires_at).toBeDefined();
    });

    test('should reject invalid credentials', async ({ request }) => {
      const response = await request.post(`${baseURL}/api/auth/login`, {
        data: {
          username: 'admin',
          password: 'wrongpassword'
        }
      });
      
      expect(response.ok()).toBeTruthy(); // HTTP OK but application error
      const data = await response.json();
      
      expect(data.success).toBe(false);
      expect(data.message).toContain('Invalid username or password');
    });

    test('should verify valid token', async ({ request }) => {
      const response = await request.get(`${baseURL}/api/auth/verify`, {
        headers: {
          'Authorization': `Bearer ${authToken}`
        }
      });
      
      expect(response.ok()).toBeTruthy();
      const data = await response.json();
      
      expect(data.success).toBe(true);
      expect(data.data.sub).toBe('admin');
      expect(data.data.role).toBe('admin');
    });

    test('should reject invalid token', async ({ request }) => {
      const response = await request.get(`${baseURL}/api/auth/verify`, {
        headers: {
          'Authorization': 'Bearer invalid-token'
        }
      });
      
      expect(response.status()).toBe(401);
    });
  });

  test.describe('File Management API', () => {
    const testFileName = 'api-test-file.txt';
    const testFileContent = `API Test File
Created: ${new Date().toISOString()}

This file was created by the API tests.

Line 1
Line 2
Line 3

Special characters: àáâãäåæçèéêë
Numbers: 123456789
Symbols: !@#$%^&*()_+-=[]{}|;:'",./<>?`;

    const testMarkdownFile = 'api-test-file.md';
    const testMarkdownContent = `# API Test Markdown

This is a **test markdown file** created by the API tests.

## Features Tested
- File creation
- File reading
- File editing
- File deletion

\`\`\`javascript
console.log('API test successful!');
\`\`\`

*Created*: ${new Date().toISOString()}
`;

    test('should create a text file', async ({ request }) => {
      const response = await request.post(`${baseURL}/api/admin/files/save/${testFileName}`, {
        headers: {
          'Authorization': `Bearer ${authToken}`
        },
        data: {
          content: testFileContent
        }
      });
      
      expect(response.ok()).toBeTruthy();
      const data = await response.json();
      
      expect(data.success).toBe(true);
      expect(data.message).toContain('saved successfully');
    });

    test('should create a markdown file', async ({ request }) => {
      const response = await request.post(`${baseURL}/api/admin/files/save/${testMarkdownFile}`, {
        headers: {
          'Authorization': `Bearer ${authToken}`
        },
        data: {
          content: testMarkdownContent
        }
      });
      
      expect(response.ok()).toBeTruthy();
      const data = await response.json();
      
      expect(data.success).toBe(true);
      expect(data.message).toContain('saved successfully');
    });

    test('should read the created text file', async ({ request }) => {
      const response = await request.get(`${baseURL}/api/admin/files/read/${testFileName}`, {
        headers: {
          'Authorization': `Bearer ${authToken}`
        }
      });
      
      expect(response.ok()).toBeTruthy();
      const data = await response.json();
      
      expect(data.success).toBe(true);
      expect(data.data.content).toBe(testFileContent);
      expect(data.data.file_type).toBe('txt');
      expect(data.data.is_editable).toBe(true);
    });

    test('should read the created markdown file', async ({ request }) => {
      const response = await request.get(`${baseURL}/api/admin/files/read/${testMarkdownFile}`, {
        headers: {
          'Authorization': `Bearer ${authToken}`
        }
      });
      
      expect(response.ok()).toBeTruthy();
      const data = await response.json();
      
      expect(data.success).toBe(true);
      expect(data.data.content).toBe(testMarkdownContent);
      expect(data.data.file_type).toBe('md');
      expect(data.data.is_editable).toBe(true);
    });

    test('should list files and include created files', async ({ request }) => {
      const response = await request.get(`${baseURL}/api/admin/files/list/`, {
        headers: {
          'Authorization': `Bearer ${authToken}`
        }
      });
      
      expect(response.ok()).toBeTruthy();
      const data = await response.json();
      
      expect(data.success).toBe(true);
      expect(Array.isArray(data.data.items)).toBe(true);
      
      const fileNames = data.data.items.map(item => item.name);
      expect(fileNames).toContain(testFileName);
      expect(fileNames).toContain(testMarkdownFile);
    });

    test('should update file content', async ({ request }) => {
      const updatedContent = testFileContent + '\n\nUPDATED: This line was added by the API test';
      
      const response = await request.post(`${baseURL}/api/admin/files/save/${testFileName}`, {
        headers: {
          'Authorization': `Bearer ${authToken}`
        },
        data: {
          content: updatedContent
        }
      });
      
      expect(response.ok()).toBeTruthy();
      const data = await response.json();
      expect(data.success).toBe(true);
      
      // Verify the update by reading the file
      const readResponse = await request.get(`${baseURL}/api/admin/files/read/${testFileName}`, {
        headers: {
          'Authorization': `Bearer ${authToken}`
        }
      });
      
      expect(readResponse.ok()).toBeTruthy();
      const readData = await readResponse.json();
      expect(readData.data.content).toBe(updatedContent);
    });

    test('should reject reading non-existent file', async ({ request }) => {
      const response = await request.get(`${baseURL}/api/admin/files/read/non-existent-file.txt`, {
        headers: {
          'Authorization': `Bearer ${authToken}`
        }
      });
      
      expect(response.status()).toBe(404);
    });

    test('should reject saving non-editable file types', async ({ request }) => {
      const response = await request.post(`${baseURL}/api/admin/files/save/test.bin`, {
        headers: {
          'Authorization': `Bearer ${authToken}`
        },
        data: {
          content: 'binary content'
        }
      });
      
      expect(response.status()).toBe(400);
      const data = await response.json();
      expect(data.success).toBe(false);
      expect(data.message).toContain('not editable');
    });

    test('should require authentication for file operations', async ({ request }) => {
      // Test without token
      const response1 = await request.get(`${baseURL}/api/admin/files/list/`);
      expect(response1.status()).toBe(401);
      
      // Test with invalid token
      const response2 = await request.get(`${baseURL}/api/admin/files/list/`, {
        headers: {
          'Authorization': 'Bearer invalid-token'
        }
      });
      expect(response2.status()).toBe(401);
    });

    test('should delete the created files', async ({ request }) => {
      // Delete text file
      const response1 = await request.post(`${baseURL}/api/admin/files/delete`, {
        headers: {
          'Authorization': `Bearer ${authToken}`
        },
        data: {
          path: testFileName
        }
      });
      
      expect(response1.ok()).toBeTruthy();
      const data1 = await response1.json();
      expect(data1.success).toBe(true);
      
      // Delete markdown file
      const response2 = await request.post(`${baseURL}/api/admin/files/delete`, {
        headers: {
          'Authorization': `Bearer ${authToken}`
        },
        data: {
          path: testMarkdownFile
        }
      });
      
      expect(response2.ok()).toBeTruthy();
      const data2 = await response2.json();
      expect(data2.success).toBe(true);
      
      // Verify files are deleted by trying to read them
      const readResponse = await request.get(`${baseURL}/api/admin/files/read/${testFileName}`, {
        headers: {
          'Authorization': `Bearer ${authToken}`
        }
      });
      expect(readResponse.status()).toBe(404);
    });
  });

  test.describe('File Rename and Move API', () => {
    const originalFile = 'rename-test-original.txt';
    const renamedFile = 'rename-test-renamed.txt';
    const testContent = 'This file will be renamed and moved';

    test('should create file for rename/move tests', async ({ request }) => {
      const response = await request.post(`${baseURL}/api/admin/files/save/${originalFile}`, {
        headers: {
          'Authorization': `Bearer ${authToken}`
        },
        data: {
          content: testContent
        }
      });
      
      expect(response.ok()).toBeTruthy();
    });

    test('should rename a file', async ({ request }) => {
      const response = await request.post(`${baseURL}/api/admin/files/rename`, {
        headers: {
          'Authorization': `Bearer ${authToken}`
        },
        data: {
          old_path: originalFile,
          new_path: renamedFile
        }
      });
      
      expect(response.ok()).toBeTruthy();
      const data = await response.json();
      expect(data.success).toBe(true);
      
      // Verify old file doesn't exist
      const oldResponse = await request.get(`${baseURL}/api/admin/files/read/${originalFile}`, {
        headers: {
          'Authorization': `Bearer ${authToken}`
        }
      });
      expect(oldResponse.status()).toBe(404);
      
      // Verify new file exists with same content
      const newResponse = await request.get(`${baseURL}/api/admin/files/read/${renamedFile}`, {
        headers: {
          'Authorization': `Bearer ${authToken}`
        }
      });
      expect(newResponse.ok()).toBeTruthy();
      const newData = await newResponse.json();
      expect(newData.data.content).toBe(testContent);
    });

    test('should download files correctly', async ({ request }) => {
      // Create a test file first
      const testContent = 'Download test file content\nLine 2\nLine 3';
      await request.post(`${baseURL}/api/admin/files/save/download-test.txt`, {
        headers: {
          'Authorization': `Bearer ${authToken}`
        },
        data: {
          content: testContent
        }
      });
      
      // Download the file
      const response = await request.get(`${baseURL}/api/admin/files/download/download-test.txt`, {
        headers: {
          'Authorization': `Bearer ${authToken}`
        }
      });
      
      expect(response.ok()).toBeTruthy();
      expect(response.headers()['content-type']).toBe('application/octet-stream');
      expect(response.headers()['content-disposition']).toContain('attachment; filename="download-test.txt"');
      
      const downloadedContent = await response.text();
      expect(downloadedContent).toBe(testContent);
      
      // Clean up
      await request.post(`${baseURL}/api/admin/files/delete`, {
        headers: {
          'Authorization': `Bearer ${authToken}`
        },
        data: {
          path: 'download-test.txt'
        }
      });
    });

    test('should clean up rename test file', async ({ request }) => {
      await request.post(`${baseURL}/api/admin/files/delete`, {
        headers: {
          'Authorization': `Bearer ${authToken}`
        },
        data: {
          path: renamedFile
        }
      });
    });
  });

  test.describe('Error Handling', () => {
    test('should handle path traversal attempts', async ({ request }) => {
      const maliciousPaths = [
        '../../../etc/passwd',
        '..\\..\\windows\\system32\\config\\sam',
        '/etc/passwd',
        'C:\\Windows\\System32\\config\\SAM'
      ];

      for (const path of maliciousPaths) {
        const response = await request.get(`${baseURL}/api/admin/files/read/${encodeURIComponent(path)}`, {
          headers: {
            'Authorization': `Bearer ${authToken}`
          }
        });
        
        // Should either be 400 (bad request) or 404 (not found), never 200
        expect(response.status()).not.toBe(200);
      }
    });

    test('should validate file operations input', async ({ request }) => {
      // Test empty content
      const response1 = await request.post(`${baseURL}/api/admin/files/save/empty-test.txt`, {
        headers: {
          'Authorization': `Bearer ${authToken}`
        },
        data: {
          content: ''
        }
      });
      expect(response1.ok()).toBeTruthy(); // Empty content should be allowed
      
      // Clean up
      await request.post(`${baseURL}/api/admin/files/delete`, {
        headers: {
          'Authorization': `Bearer ${authToken}`
        },
        data: {
          path: 'empty-test.txt'
        }
      });
    });
  });
});