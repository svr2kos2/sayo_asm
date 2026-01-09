const fs = require('fs');
const path = require('path');

// Create bin directory if it doesn't exist
const binDir = path.join(__dirname, '..', 'bin');
if (!fs.existsSync(binDir)) {
    fs.mkdirSync(binDir, { recursive: true });
}

// Source path for the LSP server
const sourceExe = path.join(__dirname, '..', '..', 'target', 'release', 'sayo-lsp.exe');
const destExe = path.join(binDir, 'sayo-lsp.exe');

// Copy the server executable
if (fs.existsSync(sourceExe)) {
    console.log(`Copying ${sourceExe} to ${destExe}`);
    fs.copyFileSync(sourceExe, destExe);
    console.log('LSP server copied successfully!');
} else {
    console.warn(`Warning: ${sourceExe} not found. Please build the LSP server first with: cargo build -p sayo_lsp --release`);
}
