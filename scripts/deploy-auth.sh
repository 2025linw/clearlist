#!/bin/sh
set -eu


readonly SERVICE_NAME="clearlist-auth.service"
readonly ARTIFACT="/tmp/clearlist-auth/clearlist-auth.tar.gz"
readonly DEPLOY="/tmp/clearlist-auth/clearlist-auth"

readonly DEPLOY_PATH="/opt/clearlist-auth"
readonly RELEASE_ROOT="$DEPLOY_PATH/releases"
readonly CURRENT_LINK="$DEPLOY_PATH/current"


# Ensure script arguments
[ "$#" -eq 3 ] || {
	echo "Usage: $0 <version> <timestamp> <github-sha>" >&2
	exit 1
}

version="$1"
timestamp="$2"
sha="$3"


# Setup artifact
if [ ! -f "$ARTIFACT" ]; then
	echo "Unable to find deployed artifact: $ARTIFACT" >&2
	exit 1
fi

rm -rf "$DEPLOY"
mkdir -p "$DEPLOY"
tar -xzf "$ARTIFACT" -C "$DEPLOY"

new_release="$RELEASE_ROOT/clearlist-auth-$version-$timestamp-$sha"
if [ -d "$new_release" ]; then
	echo "Release already exists: $new_release" >&2
	exit 1
fi

old_release=""
if [ -L "$CURRENT_LINK" ]; then
	old_release="$(readlink -f "$CURRENT_LINK")"
fi


# Function
rollback() {
	if [ -d "$old_release" ]; then
		systemctl stop "$SERVICE_NAME"
		ln -sfn "$old_release" "$CURRENT_LINK"
		systemctl start "$SERVICE_NAME"
	else
		echo "No previous release to roll back to" >&2
	fi

	exit 1
}


# Deploy new release
mkdir -p "$RELEASE_ROOT"

systemctl stop "$SERVICE_NAME"

mv "$DEPLOY" "$new_release"
ln -sfn "$new_release" "$CURRENT_LINK"

cd "$DEPLOY_PATH"

if ! node "$CURRENT_LINK/dist/migrate.js"; then
  echo "Migration failed" >&2
  rollback
fi


# Start service and verify success, otherwise rollback
systemctl start "$SERVICE_NAME"

MAX_WAIT=30
INTERVAL=2

elapsed=0
while ! systemctl is-active --quiet "$SERVICE_NAME"; do
	sleep "$INTERVAL"
	elapsed=$((elapsed + INTERVAL))

	if [ "$elapsed" -ge "$MAX_WAIT" ]; then
		echo "Service failed to start, rolling back..." >&2
		rollback
	fi
done

elapsed=0
while ! curl -fs http://127.0.0.1:9001/api/auth/ok >/dev/null 2>&1; do
	sleep "$INTERVAL"
	elapsed=$((elapsed + INTERVAL))

	if [ "$elapsed" -ge "$MAX_WAIT" ]; then
		echo "Health check failed" >&2
		rollback
	fi
done

echo "Deployment successful: $new_release"
