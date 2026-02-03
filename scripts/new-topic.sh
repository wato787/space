#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TEMPLATE_DIR="${ROOT_DIR}/topics/_template"
TOPICS_DIR="${ROOT_DIR}/topics"

usage() {
  cat <<'EOF'
Usage:
  ./scripts/new-topic.sh "<topic-name>"

Example:
  ./scripts/new-topic.sh "http-cache"
EOF
}

if [[ $# -ne 1 ]]; then
  usage
  exit 2
fi

TOPIC_NAME="$1"

if [[ -z "${TOPIC_NAME}" ]]; then
  echo "topic-name is empty" >&2
  exit 2
fi

# Avoid dangerous paths.
if [[ "${TOPIC_NAME}" == *"/"* || "${TOPIC_NAME}" == "." || "${TOPIC_NAME}" == ".." ]]; then
  echo "invalid topic-name: ${TOPIC_NAME}" >&2
  exit 2
fi

if [[ ! -d "${TEMPLATE_DIR}" ]]; then
  echo "template not found: ${TEMPLATE_DIR}" >&2
  exit 1
fi

DEST_DIR="${TOPICS_DIR}/${TOPIC_NAME}"
if [[ -e "${DEST_DIR}" ]]; then
  echo "already exists: ${DEST_DIR}" >&2
  exit 1
fi

mkdir -p "${DEST_DIR}"

# Copy template files/dirs.
cp -R "${TEMPLATE_DIR}/." "${DEST_DIR}/"

# Replace placeholder in README.
if [[ -f "${DEST_DIR}/README.md" ]]; then
  # Portable enough for GNU sed (this environment).
  sed -i "s/<topic-name>/${TOPIC_NAME}/g" "${DEST_DIR}/README.md"
fi

echo "Created: topics/${TOPIC_NAME}"
echo "Next:"
echo "  - edit: topics/${TOPIC_NAME}/README.md"
echo "  - optionally add to: topics/README.md"

