const invoke = window.__TAURI_INTERNALS__.invoke;

document.addEventListener('DOMContentLoaded', () => {
    const queryInput = document.getElementById('query-input');
    const submitButton = document.getElementById('submit-query');
    const queryResult = document.getElementById('query-result');

    const executeQuery = async () => {
        const query = queryInput.value.trim();
        if (!query) {
            queryResult.textContent = 'Please enter a query.';
            queryResult.style.color = 'var(--primary-text)';
            return;
        }

        queryResult.textContent = 'Executing query...';
        queryResult.style.color = 'var(--primary-text)';

        try {
            // The 'execute_query' command is defined in the Rust backend.
            const result = await invoke('execute_query', { query });
            queryResult.textContent = result || '[No output from server]';
        } catch (error) {
            queryResult.textContent = `Error:\n${error}`;
            queryResult.style.color = '#ff5c5c';
        }
    };

    submitButton.addEventListener('click', executeQuery);
    
    // Allow submitting with Ctrl+Enter or Cmd+Enter in the textarea for convenience.
    queryInput.addEventListener('keydown', (e) => {
        if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
            e.preventDefault(); // Prevent default Enter behavior (new line)
            executeQuery();
        }
    });
});
