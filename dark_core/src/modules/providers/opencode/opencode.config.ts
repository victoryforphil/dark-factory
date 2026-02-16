import { z } from 'zod';

import type { ConfigSubsystemDefinition } from '../../../config/lib/types';

const opencodeLogLevelSchema = z.enum(['DEBUG', 'INFO', 'WARN', 'ERROR']);

/** OpenCode SDK/runtime config owned by the provider domain. */
export const opencodeConfigSection = {
  shape: {
    hostname: z.string().min(1).default('127.0.0.1'),
    port: z.number().int().min(1).max(65_535).default(4096),
    startupTimeoutMs: z.number().int().min(100).max(120_000).default(5000),
    autoStartServer: z.boolean().default(true),
    logLevel: opencodeLogLevelSchema.default('INFO'),
    tuiCommand: z.string().min(1).default('opencode'),
  },
  env: [
    { path: 'opencode.hostname', env: 'DARKFACTORY_OPENCODE_HOST' },
    { path: 'opencode.port', env: 'DARKFACTORY_OPENCODE_PORT' },
    { path: 'opencode.startupTimeoutMs', env: 'DARKFACTORY_OPENCODE_STARTUP_TIMEOUT_MS' },
    { path: 'opencode.autoStartServer', env: 'DARKFACTORY_OPENCODE_AUTO_START_SERVER' },
    { path: 'opencode.logLevel', env: 'DARKFACTORY_OPENCODE_LOG_LEVEL' },
    { path: 'opencode.tuiCommand', env: 'DARKFACTORY_OPENCODE_TUI_COMMAND' },
  ],
} satisfies ConfigSubsystemDefinition;
