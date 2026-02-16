# syntax=docker/dockerfile:1.7

ARG UBUNTU_VERSION=24.04
ARG USERNAME=dark
ARG USER_UID=1000
ARG USER_GID=1000

FROM ubuntu:${UBUNTU_VERSION} AS common

ARG USERNAME
ARG USER_UID
ARG USER_GID

ENV DEBIAN_FRONTEND=noninteractive
ENV BUN_INSTALL=/opt/bun
ENV PATH="${BUN_INSTALL}/bin:${PATH}"

RUN apt-get update && apt-get install -y --no-install-recommends \
  bash \
  ca-certificates \
  curl \
  git \
  gzip \
  tini \
  unzip \
  xz-utils \
  && rm -rf /var/lib/apt/lists/*

RUN mkdir -p "${BUN_INSTALL}" \
  && curl -fsSL https://bun.sh/install | bash -s -- bun-v1.3.9 \
  && ln -sf "${BUN_INSTALL}/bin/bun" /usr/local/bin/bun \
  && ln -sf "${BUN_INSTALL}/bin/bunx" /usr/local/bin/bunx

RUN if ! getent group "${USERNAME}" >/dev/null; then groupadd "${USERNAME}"; fi \
  && if ! id -u "${USERNAME}" >/dev/null 2>&1; then useradd -m -s /bin/bash -g "${USERNAME}" "${USERNAME}"; fi

WORKDIR /workspace

FROM common AS build

ENV PROTO_HOME=/opt/proto
ENV PATH="${PROTO_HOME}/bin:${PROTO_HOME}/shims:${PATH}"

RUN apt-get update && apt-get install -y --no-install-recommends \
  build-essential \
  libssl-dev \
  pkg-config \
  python3 \
  sqlite3 \
  && rm -rf /var/lib/apt/lists/*

RUN mkdir -p "${PROTO_HOME}" \
  && curl -fsSL https://moonrepo.dev/install/proto.sh | bash -s -- --yes --no-profile

COPY .prototools /workspace/.prototools
RUN proto install

COPY . /workspace

RUN bun install --cwd dark_core --frozen-lockfile
RUN ln -sfn /workspace/dark_core/node_modules /workspace/node_modules
ENV PATH="/workspace/dark_core/node_modules/.bin:${PATH}"
RUN moon run dark_core:build

FROM common AS run

ENV NODE_ENV=production

WORKDIR /workspace/dark_core

COPY --from=build /workspace/dark_core/dist /workspace/dark_core/dist
COPY --from=build /workspace/dark_core/node_modules /workspace/dark_core/node_modules
COPY --from=build /workspace/dark_core/package.json /workspace/dark_core/package.json
COPY --from=build /workspace/generated /workspace/generated
COPY --from=build /workspace/prisma /workspace/prisma

EXPOSE 4150

ENTRYPOINT ["tini", "--"]
CMD ["bun", "dist/index.js"]

FROM build AS agentbox

RUN apt-get update && apt-get install -y --no-install-recommends \
  fd-find \
  gh \
  jq \
  less \
  procps \
  ripgrep \
  && rm -rf /var/lib/apt/lists/*

RUN ln -sf /usr/bin/fdfind /usr/local/bin/fd

ENV TERM=xterm-256color

ENTRYPOINT ["tini", "--"]
CMD ["sleep", "infinity"]

FROM build AS ci

ENV CI=true
WORKDIR /workspace

ENTRYPOINT ["tini", "--"]
CMD ["moon", "run", "dark_core:test"]

FROM agentbox AS devcontainer

ARG USERNAME

RUN apt-get update && apt-get install -y --no-install-recommends \
  fzf \
  neovim \
  tmux \
  zsh \
  && rm -rf /var/lib/apt/lists/*

COPY docker/devcontainer/.zshrc /home/${USERNAME}/.zshrc
COPY docker/devcontainer/.tmux.conf /home/${USERNAME}/.tmux.conf

RUN chown ${USERNAME}:${USERNAME} /home/${USERNAME}/.zshrc /home/${USERNAME}/.tmux.conf

USER ${USERNAME}
WORKDIR /workspace

ENTRYPOINT ["tini", "--"]
CMD ["sleep", "infinity"]
