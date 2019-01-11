#!/bin/bash

set -euo pipefail

id -g rust &> /dev/null || groupadd --g ${GID} rust
id -u rust &> /dev/null || useradd -u ${UID} -g ${GID} -ms /bin/bash rust

chown -R ${UID}:${GID} /workdir
chown -R ${UID}:${GID} /usr/local/cargo

su-exec ${UID}:${GID} "$@"
