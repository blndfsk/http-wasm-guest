#!/usr/bin/env bash
set -eu -o pipefail

function cleanup()
{
    podman pod rm -f $pod
    podman image rm localhost/$plugin
}

trap 'cleanup' EXIT HUP INT TERM

plugin=$1
cargo build --target wasm32-wasip1 --example $plugin

container=$(buildah from traefik:v3.6)
buildah copy $container target/wasm32-wasip1/debug/examples/$plugin.wasm /opt/traefik/plugins-local/src/$plugin/plugin.wasm
buildah copy $container examples/$plugin.yml /opt/traefik/plugins-local/src/$plugin/.traefik.yml
entrypoint=$(
cat<<EOF
[ "traefik",
  "--entrypoints.web.address=:8080",
  "--experimental.localplugins.$plugin.modulename=$plugin",
  "--providers.docker=true",
  "--log.level=INFO"
]
EOF
)
buildah config --workingdir "/opt/traefik" $container
buildah config --entrypoint "$entrypoint" $container
buildah commit $container localhost/$plugin
buildah rm $container

pod=$(podman pod create -p 8080:8080)
podman run -d --pod $pod --replace --name whoami \
    --label 'traefik.http.routers.whoami.rule=Host(`whoami.localhost`)' \
    --label "traefik.http.routers.whoami.middlewares=$plugin" \
    --label "traefik.http.routers.whoami.service=whoami" \
    --label "traefik.http.middlewares.$plugin.plugin.$plugin" \
    --label "traefik.http.services.whoami.loadbalancer.server.url=http://localhost:8081" \
    traefik/whoami -port 8081

podman run -it --rm --pod $pod \
    --volume /run/user/${UID}/podman/podman.sock:/var/run/docker.sock \
    localhost/$plugin
