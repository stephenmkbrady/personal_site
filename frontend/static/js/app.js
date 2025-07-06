const API_BASE = '/api';

// Utility function to make API calls
async function apiCall(endpoint, options = {}) {
    const url = `${API_BASE}${endpoint}`;
    const defaultOptions = {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
        },
    };
    
    try {
        const response = await fetch(url, { ...defaultOptions, ...options });
        const data = await response.json();
        return { success: response.ok, data, status: response.status };
    } catch (error) {
        return { success: false, error: error.message, status: 0 };
    }
}

// Update result display
function updateResult(elementId, result, isSuccess = true) {
    const element = document.getElementById(elementId);
    element.className = `result ${isSuccess ? 'success' : 'error'}`;
    element.textContent = JSON.stringify(result, null, 2);
}

function setLoading(elementId, message = 'Loading...') {
    const element = document.getElementById(elementId);
    element.className = 'result loading';
    element.textContent = message;
}

// Test functions
async function testHealthCheck() {
    setLoading('health-result');
    const result = await apiCall('/health');
    updateResult('health-result', result, result.success);
}

async function testContentList() {
    const category = document.getElementById('content-category').value;
    setLoading('content-list-result');
    const result = await apiCall(`/content/${category}`);
    updateResult('content-list-result', result, result.success);
}

async function testContentItem() {
    const category = document.getElementById('content-item-category').value;
    const slug = document.getElementById('content-item-slug').value;
    
    if (!category || !slug) {
        updateResult('content-item-result', { error: 'Category and slug are required' }, false);
        return;
    }
    
    setLoading('content-item-result');
    const result = await apiCall(`/content/${category}/${slug}`);
    updateResult('content-item-result', result, result.success);
}

async function testContentTags() {
    setLoading('content-tags-result');
    const result = await apiCall('/content/tags');
    updateResult('content-tags-result', result, result.success);
}

async function testGitHubProjects() {
    setLoading('github-projects-result');
    const result = await apiCall('/github/projects');
    updateResult('github-projects-result', result, result.success);
}

async function testRefreshGitHub() {
    setLoading('github-refresh-result');
    const result = await apiCall('/admin/refresh-github', { method: 'POST' });
    updateResult('github-refresh-result', result, result.success);
}

async function testAllEndpoints() {
    const allTestsElement = document.getElementById('all-tests-result');
    allTestsElement.className = 'result loading';
    allTestsElement.textContent = 'Running all tests...\n\n';
    
    const tests = [
        { name: 'Health Check', fn: () => apiCall('/health') },
        { name: 'Content List (projects)', fn: () => apiCall('/content/project') },
        { name: 'Content List (blog)', fn: () => apiCall('/content/blog') },
        { name: 'Content Tags', fn: () => apiCall('/content/tags') },
        { name: 'GitHub Projects', fn: () => apiCall('/github/projects') },
        { name: 'Refresh GitHub Cache', fn: () => apiCall('/admin/refresh-github', { method: 'POST' }) },
    ];
    
    let results = '';
    let allPassed = true;
    
    for (const test of tests) {
        allTestsElement.textContent += `Testing ${test.name}...\n`;
        const result = await test.fn();
        const status = result.success ? 'PASS' : 'FAIL';
        
        results += `${test.name}: ${status}\n`;
        if (result.success) {
            results += `  Response: ${JSON.stringify(result.data, null, 2)}\n`;
        } else {
            results += `  Error: ${result.error || 'Unknown error'}\n`;
            results += `  Status: ${result.status}\n`;
            allPassed = false;
        }
        results += '\n';
    }
    
    allTestsElement.className = `result ${allPassed ? 'success' : 'error'}`;
    allTestsElement.textContent = `All tests completed!\n\n${results}`;
}

// Initialize page
document.addEventListener('DOMContentLoaded', function() {
    console.log('Portfolio API Tester loaded');
    
    // Set default values
    document.getElementById('content-item-category').value = 'project';
    document.getElementById('content-item-slug').value = 'project1';
    
    // Test health check on load
    testHealthCheck();
});