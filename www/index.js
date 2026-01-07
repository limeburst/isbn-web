import init, { parse_isbn } from './pkg/isbn_web.js';

async function main() {
    await init();

    const input = document.getElementById('isbn-input');
    const results = document.getElementById('results');
    const error = document.getElementById('error');

    const statusEl = document.getElementById('status');
    const typeEl = document.getElementById('type');
    const hyphenatedEl = document.getElementById('hyphenated');
    const groupEl = document.getElementById('group');
    const isbn10El = document.getElementById('isbn10');
    const isbn13El = document.getElementById('isbn13');

    function updateResults() {
        const value = input.value.trim();

        if (!value) {
            results.classList.add('hidden');
            error.classList.add('hidden');
            return;
        }

        const info = parse_isbn(value);

        if (info.valid) {
            results.classList.remove('hidden');
            error.classList.add('hidden');

            statusEl.textContent = 'Valid';
            statusEl.className = 'value valid';

            typeEl.textContent = info.isbn_type || '-';
            hyphenatedEl.textContent = info.hyphenated || '-';
            groupEl.textContent = info.registration_group || '-';
            isbn10El.textContent = info.isbn10 || 'N/A (979 prefix)';
            isbn13El.textContent = info.isbn13 || '-';
        } else {
            results.classList.remove('hidden');
            error.classList.add('hidden');

            statusEl.textContent = 'Invalid';
            statusEl.className = 'value invalid';

            typeEl.textContent = '-';
            hyphenatedEl.textContent = '-';
            groupEl.textContent = '-';
            isbn10El.textContent = '-';
            isbn13El.textContent = '-';
        }
    }

    input.addEventListener('input', updateResults);

    // Handle sample button clicks
    document.querySelectorAll('.sample').forEach(btn => {
        btn.addEventListener('click', () => {
            input.value = btn.dataset.isbn;
            updateResults();
            input.focus();
        });
    });
}

main();
