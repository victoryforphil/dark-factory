import { z } from 'zod';

import type { ConfigSubsystemDefinition } from '../../config/lib/types';

const sshHostConfigSchema = z.object({
  name: z.string().min(1).optional(),
  host: z.string().min(1),
  user: z.string().min(1).optional(),
  port: z.number().int().min(1).max(65_535).optional(),
  defaultPath: z.string().min(1).optional(),
  sshArgs: z.array(z.string().min(1)).default([]),
});

const sshPortForwardPresetSchema = z.object({
  name: z.string().min(1),
  host: z.string().min(1).optional(),
  localPort: z.number().int().min(1).max(65_535),
  remotePort: z.number().int().min(1).max(65_535),
  remoteHost: z.string().min(1).default('127.0.0.1'),
  description: z.string().min(1).optional(),
  sshArgs: z.array(z.string().min(1)).default([]),
});

export const sshConfigSection = {
  shape: {
    defaultSshArgs: z.array(z.string().min(1)).default([]),
    hosts: z.array(sshHostConfigSchema).default([]),
    portForwards: z.array(sshPortForwardPresetSchema).default([]),
  },
  env: [],
} satisfies ConfigSubsystemDefinition;
