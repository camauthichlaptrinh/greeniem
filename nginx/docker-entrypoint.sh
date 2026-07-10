#!/bin/sh
# Runs automatically before nginx starts (official nginx image convention:
# every executable script in /docker-entrypoint.d/ is sourced on boot).
# Writes the backend URL into a small JS file so the same image can point
# at any environment's API without being rebuilt.
set -eu

cat > /usr/share/nginx/html/env.js <<EOF
window.__API_BASE__ = "${API_BASE:-}";
EOF
