import { z } from 'zod';

import type { ConfigSubsystemDefinition } from '../../../config/lib/types';

const opencodeLogLevelSchema = z.enum(['DEBUG', 'INFO', 'WARN', 'ERROR']);
const opencodeServerModeSchema = z.enum(['serve', 'web']);

/** OpenCode SDK/runtime config owned by the provider domain. */
export const opencodeServerConfigSection = {
  shape: {
    hostname: z.string().min(1).default('127.0.0.1'),
    port: z.number().int().min(1).max(65_535).default(4096),
    startupTimeoutMs: z.number().int().min(100).max(120_000).default(5000),
    autoStartServer: z.boolean().default(true),
    serverMode: opencodeServerModeSchema.default('serve'),
    tmuxSessionName: z.string().min(1).default('dark-opencode-server'),
    logLevel: opencodeLogLevelSchema.default('INFO'),
    tuiCommand: z.string().min(1).default('opencode'),
    includeRecentSessionsWhenStatusEmpty: z.boolean().default(true),
    recentSessionWindowHours: z.number().int().min(1).max(24 * 30).default(72),
    recentSessionLimit: z.number().int().min(1).max(500).default(50),
  },
  env: [
    { path: 'opencode.hostname', env: 'DARKFACTORY_OPENCODE_HOST' },
    { path: 'opencode.port', env: 'DARKFACTORY_OPENCODE_PORT' },
    { path: 'opencode.startupTimeoutMs', env: 'DARKFACTORY_OPENCODE_STARTUP_TIMEOUT_MS' },
    { path: 'opencode.autoStartServer', env: 'DARKFACTORY_OPENCODE_AUTO_START_SERVER' },
    { path: 'opencode.serverMode', env: 'DARKFACTORY_OPENCODE_SERVER_MODE' },
    { path: 'opencode.tmuxSessionName', env: 'DARKFACTORY_OPENCODE_TMUX_SESSION_NAME' },
    { path: 'opencode.logLevel', env: 'DARKFACTORY_OPENCODE_LOG_LEVEL' },
    { path: 'opencode.tuiCommand', env: 'DARKFACTORY_OPENCODE_TUI_COMMAND' },
    {
      path: 'opencode.includeRecentSessionsWhenStatusEmpty',
      env: 'DARKFACTORY_OPENCODE_INCLUDE_RECENT_WHEN_STATUS_EMPTY',
    },
    { path: 'opencode.recentSessionWindowHours', env: 'DARKFACTORY_OPENCODE_RECENT_SESSION_HOURS' },
    { path: 'opencode.recentSessionLimit', env: 'DARKFACTORY_OPENCODE_RECENT_SESSION_LIMIT' },
  ],
} satisfies ConfigSubsystemDefinition;
