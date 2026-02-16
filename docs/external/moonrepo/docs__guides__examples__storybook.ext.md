----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/guides/examples/storybook
- Keywords: moon, moonrepo, docs, monorepo, build, guides, examples, storybook
- Summary: Storybook is a frontend workshop for building UI components and pages in isolation. Thousands of
----

Source: https://moonrepo.dev/docs/guides/examples/storybook

# Storybook example

Storybook is a frontend workshop for building UI components and pages in isolation. Thousands of
teams use it for UI development, testing, and documentation. It’s open source and free.

[Storybook v7](https://storybook.js.org/docs/7.0) is typically coupled with
[Vite](https://vitejs.dev/). To scaffold a new Storybook project with Vite, run the following
command in a project root. This guide assumes you are using React, however it is possible to use
almost any (meta) framework with Storybook.

```
cd
 && npx storybook init
```

We highly suggest reading our documentation on [using Vite (and Vitest) with moon](/docs/guides/examples/vite) and
[using Jest with moon](/docs/guides/examples/jest) for a more holistic view.

## Setup

This section assumes Storybook is being used with Vite, and is integrated on a per-project basis.

After setting up Storybook, ensure [`moon.yml`](/docs/config/project) has the following tasks:

/moon.yml

```
fileGroups:  storybook:    - 'src/**/*'    - 'stories/**/*'    - 'tests/**/*'    - '.storybook/**/*'tasks:  buildStorybook:    command: 'build-storybook --output-dir @out(0)'    inputs:      - '@group(storybook)'    outputs:      - 'build'  storybook:    preset: 'server'    command: 'start-storybook'    inputs:      - '@group(storybook)'
```

To run the Storybook development server:

```
moon run
:storybook
```

### Vite integration

Storybook 7 uses Vite out of the box, and as such, no configuration is required, but should you
choose to extend the Vite config, you can do so by passing in `viteFinal`:

.storybook/main.ts

```
import { mergeConfig } from 'vite';export default {  stories: ['../stories/**/*.stories.mdx', '../stories/**/*.stories.@(js|jsx|ts|tsx)'],  addons: ['@storybook/addon-links', '@storybook/addon-essentials'],  core: {    builder: '@storybook/builder-vite',  },  async viteFinal(config) {    // Merge custom configuration into the default config    return mergeConfig(config, {      // Use the same "resolve" configuration as your app      resolve: (await import('../vite.config.js')).default.resolve,      // Add dependencies to pre-optimization      optimizeDeps: {        include: ['storybook-dark-mode'],      },    });  },};
```

For more information on how to integrate Vite with Storybook see the
[relevant documentation](https://storybook.js.org/docs/7.0/react/builders/vite#configuration).

### Webpack integration

If you want to use Webpack with your Storybook project, you can do so by installing the relevant
package and updating configuration.

- Yarn
- Yarn (classic)
- npm
- pnpm
- Bun

```
yarn workspace
 add --dev @storybook/builder-webpack5
```

```
yarn workspace
 add --dev @storybook/builder-webpack5
```

```
npm install --save-dev --workspace
 @storybook/builder-webpack5
```

```
pnpm add --save-dev --filter
 @storybook/builder-webpack5
```

```
bun install --dev @storybook/builder-webpack5
```

.storybook/main.ts

```
export default {  core: {    builder: '@storybook/builder-webpack5',  },};
```

For more information on how to integrate Webpack with Storybook, see the
[relevant documentation](https://storybook.js.org/docs/7.0/react/builders/webpack).

### Jest integration

You can use Jest to test your stories, but isn't a requirement. Storybook ships with first-party
plugins for improved developer experience.

Install the test runner and any relevant packages:

- Yarn
- Yarn (classic)
- npm
- pnpm
- Bun

```
yarn workspace
 add --dev @storybook/addon-interactions @storybook/addon-coverage @storybook/jest@next @storybook/testing-library@next @storybook/test-runner@next
```

```
yarn workspace
 add --dev @storybook/addon-interactions @storybook/addon-coverage @storybook/jest@next @storybook/testing-library@next @storybook/test-runner@next
```

```
npm install --save-dev --workspace
 @storybook/addon-interactions @storybook/addon-coverage @storybook/jest@next @storybook/testing-library@next @storybook/test-runner@next
```

```
pnpm add --save-dev --filter
 @storybook/addon-interactions @storybook/addon-coverage @storybook/jest@next @storybook/testing-library@next @storybook/test-runner@next
```

```
bun install --dev @storybook/addon-interactions @storybook/addon-coverage @storybook/jest@next @storybook/testing-library@next @storybook/test-runner@next
```

Add the test task to your project:

/moon.yml

```
tasks:  testStorybook:    command: 'test-storybook'    inputs:      - '@group(storybook)'
```

Then enable plugins and interactions in your Storybook project:

.storybook/main.ts

```
export default {  stories: ['../src/**/*.stories.mdx', '../src/**/*.stories.@(js|jsx|ts|tsx)'],  addons: [    // Other Storybook addons    '@storybook/addon-interactions', // Addon is registered here    '@storybook/addon-coverage',  ],  features: {    interactionsDebugger: true, // Enable playback controls  },};
```

You can now start writing your tests. For an extended guide on how to write tests within your
stories, see
[writing an interaction test](https://storybook.js.org/docs/react/writing-tests/interaction-testing#write-an-interaction-test)
on the Storybook docs.

## Configuration

Storybook requires a `.storybook` folder relative to the project root. Because of this, Storybook
should be scaffolded in each project individually. Configuration may be shared through package
imports.

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
