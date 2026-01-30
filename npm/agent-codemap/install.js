// ============================================================
// Postinstall: Verify Platform Package
// ============================================================

const PLATFORMS = {
  "darwin-arm64": "@nekocode/agent-codemap-darwin-arm64",
  "darwin-x64": "@nekocode/agent-codemap-darwin-x64",
  "linux-x64": "@nekocode/agent-codemap-linux-x64",
  "win32-x64": "@nekocode/agent-codemap-win32-x64",
};

const key = `${process.platform}-${process.arch}`;
const pkg = PLATFORMS[key];

if (!pkg) {
  console.warn(`[agent-codemap] Warning: Unsupported platform ${key}`);
  console.warn(`[agent-codemap] Supported: ${Object.keys(PLATFORMS).join(", ")}`);
  process.exit(0);
}

try {
  require.resolve(`${pkg}/package.json`);
} catch {
  console.warn(`[agent-codemap] Warning: Platform package ${pkg} not installed`);
  console.warn(`[agent-codemap] This may happen if npm failed to install optional dependencies`);
}
