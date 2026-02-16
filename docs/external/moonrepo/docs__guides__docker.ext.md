----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/guides/docker
- Keywords: moon, moonrepo, docs, monorepo, build, guides, docker
- Summary: Using [Docker](https://www.docker.com/) to run your applications? Or build your artifacts? No
----

Source: https://moonrepo.dev/docs/guides/docker

# Docker integration

Using [Docker](https://www.docker.com/) to run your applications? Or build your artifacts? No
worries, moon can be utilized with Docker, and supports a robust integration layer.

success

Looking to speed up your Docker builds? Want to build in the cloud?
[Give Depot a try](https://depot.dev?ref=moonrepo)!

## Requirements

The first requirement, which is very important, is adding `.moon/cache` to the workspace root
`.dockerignore` (moon assumes builds are running from the root). Not all files in `.moon/cache` are
portable across machines/environments, so copying these file into Docker will definitely cause
interoperability issues.

.dockerignore

```
.moon/cache
```

The other requirement depends on how you want to integrate Git with Docker. Since moon executes
`git` commands under the hood, there are some special considerations to be aware of when running
moon within Docker. There's 2 scenarios to choose from:

- (recommended) Add the `.git` folder to `.dockerignore`, so that it's not `COPY`'d. moon will continue to work just fine, albeit with some functionality disabled, like caching.

- Ensure that the `git` library is installed in the container, and copy the `.git` folder with `COPY`. moon will work with full functionality, but it will increase the overall size of the image because of caching.

## Creating a `Dockerfile`

info

Our [`moon docker file`](/docs/commands/docker/file) command can automatically generate a `Dockerfile` based on this
guide! We suggest generating the file then reading the guide below to understand what's going on.

We're very familiar with how tedious `Dockerfile`s are to write and maintain, so in an effort to
reduce this headache, we've built a handful of tools to make this process much easier. With moon,
we'll take advantage of Docker's layer caching and staged builds as much as possible.

With that being said, there's many approaches you can utilize, depending on your workflow (we'll
document them below):

- Running `moon docker` commands before running `docker run|build` commands.

- Running `moon docker` commands within the `Dockerfile`.

- Using multi-staged or non-staged (standard) builds.

- Something else unique to your setup!

warning

This guide and our Docker approach is merely a suggestion and is not a requirement for using moon
with Docker! Feel free to use this as a starting point, or not at all. Choose the approach that
works best for you!

### What we're trying to avoid

Before we dive into writing a perfect `Dockerfile`, we'll briefly talk about the pain points we're
trying to avoid. In the context of Node.js and monorepo's, you may be familiar with having to `COPY`
each individual `package.json` in the monorepo before installing `node_modules`, to effectively use
layer caching. This is very brittle, as each new application or package is created, every
`Dockerfile` in the monorepo will need to be modified to account for this new `package.json`.

Furthermore, we'll have to follow a similar process for only copying source files necessary for
the build or `CMD` to complete. This is very tedious, so most developers simply use `COPY . .` and
forget about it. Copying the entire monorepo is costly, especially as it grows.

As an example, we'll use moon's official repository. The `Dockerfile` would look something like the
following.

```
FROM node:latestWORKDIR /app# Install moon binaryRUN npm install -g @moonrepo/cli# Copy moon filesCOPY ./.moon ./.moon# Copy all package.json's and lockfilesCOPY ./packages/cli/package.json ./packages/cli/package.jsonCOPY ./packages/core-linux-arm64-gnu/package.json ./packages/core-linux-arm64-gnu/package.jsonCOPY ./packages/core-linux-arm64-musl/package.json ./packages/core-linux-arm64-musl/package.jsonCOPY ./packages/core-linux-x64-gnu/package.json ./packages/core-linux-x64-gnu/package.jsonCOPY ./packages/core-linux-x64-musl/package.json ./packages/core-linux-x64-musl/package.jsonCOPY ./packages/core-macos-arm64/package.json ./packages/core-macos-arm64/package.jsonCOPY ./packages/core-macos-x64/package.json ./packages/core-macos-x64/package.jsonCOPY ./packages/core-windows-x64-msvc/package.json ./packages/core-windows-x64-msvc/package.jsonCOPY ./packages/runtime/package.json ./packages/runtime/package.jsonCOPY ./packages/types/package.json ./packages/types/package.jsonCOPY ./package.json ./package.jsonCOPY ./yarn.lock ./yarn.lockCOPY ./.yarn ./.yarnCOPY ./.yarnrc.yml ./yarnrc.yml# Install toolchain and dependencies# In non-moon repos: yarn installRUN moon docker setup# Copy project and required files# Or COPY . .COPY ./packages/types ./packages/typesCOPY ./packages/runtime ./packages/runtime# Build the targetRUN moon run runtime:build
```

For such a small monorepo, this already looks too confusing!!! Let's remedy this by utilizing moon
itself to the fullest!

### Scaffolding the bare minimum

The first step in this process is to only copy the bare minimum of files necessary for installing
dependencies (Node.js modules, etc). This is typically manifests (`package.json`), lockfiles
(`yarn.lock`, etc), and any configuration (`.yarnrc.yml`, etc).

This can all be achieved with the [`moon docker scaffold`](/docs/commands/docker/scaffold) command, which scaffolds a
skeleton of the repository structure, with only necessary files (the above). Let's update our
`Dockerfile` usage.

- Non-staged
- Multi-staged

This assumes [`moon docker scaffold `](/docs/commands/docker/scaffold) is ran outside of the `Dockerfile`.

```
FROM node:latestWORKDIR /app# Install moon binaryRUN npm install -g @moonrepo/cli# Copy workspace skeletonCOPY ./.moon/docker/workspace .# Install toolchain and dependenciesRUN moon docker setup
```

```
#### BASEFROM node:latest AS baseWORKDIR /app# Install moon binaryRUN npm install -g @moonrepo/cli#### SKELETONFROM base AS skeleton# Copy entire repository and scaffoldCOPY . .RUN moon docker scaffold
#### BUILDFROM base AS build# Copy workspace skeletonCOPY --from=skeleton /app/.moon/docker/workspace .# Install toolchain and dependenciesRUN moon docker setup
```

And with this, our dependencies will be layer cached effectively! Let's now move onto copying source
files.

### Copying necessary source files

The next step is to copy all source files necessary for `CMD` or any `RUN` commands to execute
correctly. This typically requires copying all source files for the project and all source files
of the project's dependencies... NOT the entire repository!

Luckily our [`moon docker scaffold `](/docs/commands/docker/scaffold) command has already done this for us! Let's
continue updating our `Dockerfile` to account for this, by appending the following:

- Non-staged
- Multi-staged

```
# Copy source filesCOPY ./.moon/docker/sources .# Build something (optional)RUN moon run
:
```

```
# Copy source filesCOPY --from=skeleton /app/.moon/docker/sources .# Build something (optional)RUN moon run
:
```

info

If you need to copy additional files for your commands to run successfully, you can configure the
`docker.scaffold.include` setting in [`.moon/workspace.yaml`](/docs/config/workspace#scaffold) (entire
workspace) or [`moon.yml`](/docs/config/project#scaffold) (per project).

### Pruning extraneous files

Now that we've ran a command or built an artifact, we should prune the Docker environment to remove
unneeded files and folders. We can do this with the [`moon docker prune`](/docs/commands/docker/prune) command, which
must be ran within the context of a `Dockerfile`!

```
# Prune workspaceRUN moon docker prune
```

When ran, this command will do the following, in order:

- Remove extraneous dependencies (`node_modules`) for unfocused projects.

- Install production only dependencies for the projects that were scaffolded.

info

This process can be customized using the `docker.prune` setting in
[`.moon/workspace.yaml`](/docs/config/workspace#prune).

### Final result

And with this moon integration, we've reduced the original `Dockerfile` of 35 lines to 18 lines, a
reduction of almost 50%. The original file can also be seen as `O(n)`, as each new manifest requires
cascading updates, while the moon approach is `O(1)`!

- Non-staged
- Multi-staged

```
FROM node:latestWORKDIR /app# Install moon binaryRUN npm install -g @moonrepo/cli# Copy workspace skeletonCOPY ./.moon/docker/workspace .# Install toolchain and dependenciesRUN moon docker setup# Copy source filesCOPY ./.moon/docker/sources .# Build something (optional)RUN moon run
:# Prune workspaceRUN moon docker prune# CMD
```

```
#### BASEFROM node:latest AS baseWORKDIR /app# Install moon binaryRUN npm install -g @moonrepo/cli#### SKELETONFROM base AS skeleton# Copy entire repository and scaffoldCOPY . .RUN moon docker scaffold
#### BUILDFROM base AS build# Copy workspace skeletonCOPY --from=skeleton /app/.moon/docker/workspace .# Install toolchain and dependenciesRUN moon docker setup# Copy source filesCOPY --from=skeleton /app/.moon/docker/sources .# Build something (optional)RUN moon run
:# Prune workspaceRUN moon docker prune# CMD
```

## Running `docker` commands

When running `docker` commands, they must be ran from moon's workspace root (typically the
repository root) so that the project graph and all `moon docker` commands resolve correctly.

```
docker build .
```

If you're `Dockerfile`s are located within each applicable project, use the `-f` argument.

```
docker run -f ./apps/client/Dockerfile .
```

## Troubleshooting

### Supporting `node:alpine` images

If you're trying to use the `node:alpine` image with moon's
[integrated toolchain](/docs/concepts/toolchain), you'll need to set the `MOON_TOOLCHAIN_FORCE_GLOBALS`
environment variable in the Docker image to disable moon's toolchain. This is required as Node.js
does not provide pre-built binaries for the Alpine target, so installing the Node.js toolchain will
fail.

```
FROM node:alpineENV MOON_TOOLCHAIN_FORCE_GLOBALS=true
```

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
