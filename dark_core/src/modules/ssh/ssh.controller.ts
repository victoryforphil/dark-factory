import { readFileSync } from 'node:fs';
import { homedir } from 'node:os';
import { join } from 'node:path';

import { getConfig } from '../../config';

export interface SshHostInfo {
  key: string;
  host: string;
  source: 'config' | 'ssh_config';
  label: string;
  user?: string;
  port?: number;
  defaultPath?: string;
}

export interface SshPortForwardPreset {
  name: string;
  host?: string;
  localPort: number;
  remotePort: number;
  remoteHost: string;
  description?: string;
}

export interface SshCommandInvocation {
  hostKey: string;
  destination: string;
  args: string[];
}

export interface PortForwardSpec {
  localPort: number;
  remotePort: number;
  remoteHost?: string;
}

export interface StartSshPortForwardInput {
  host: string;
  forwards: PortForwardSpec[];
  sessionName?: string;
  extraSshArgs?: string[];
}

export interface StartSshPortForwardResult {
  sessionName: string;
  host: string;
  command: string;
  forwardSpecs: string[];
  alreadyRunning: boolean;
}

export interface TmuxSessionInfo {
  name: string;
  attached: boolean;
  windows: number;
  currentCommand: string;
}

interface CommandResult {
  ok: boolean;
  stdout: string;
  stderr: string;
  exitCode: number | null;
}

const shellQuote = (value: string): string => {
  if (value.length === 0) {
    return "''";
  }

  if (/^[A-Za-z0-9_./:@%+,-]+$/.test(value)) {
    return value;
  }

  return `'${value.replace(/'/g, `'\\''`)}'`;
};

const shellJoin = (parts: readonly string[]): string => {
  return parts.map(shellQuote).join(' ');
};

const decodeBytes = (value: Uint8Array | undefined): string => {
  if (!value) {
    return '';
  }

  return new TextDecoder().decode(value).trim();
};

const runCommand = (cmd: string[]): CommandResult => {
  const result = Bun.spawnSync({
    cmd,
    stdout: 'pipe',
    stderr: 'pipe',
  });

  return {
    ok: result.success,
    stdout: decodeBytes(result.stdout),
    stderr: decodeBytes(result.stderr),
    exitCode: result.exitCode,
  };
};

const runTmux = (args: string[]): CommandResult => {
  return runCommand(['tmux', ...args]);
};

const ensureTmuxAvailable = (): void => {
  const result = runTmux(['-V']);
  if (result.ok) {
    return;
  }

  throw new Error(
    `SSH // Port Forward // tmux unavailable (stderr=${result.stderr || '<empty>'},exit=${result.exitCode ?? 'null'})`,
  );
};

const tmuxSessionExists = (sessionName: string): boolean => {
  return runTmux(['has-session', '-t', sessionName]).ok;
};

const stripInlineComment = (line: string): string => {
  return line.split('#', 1)[0] ?? '';
};

const splitOnceWhitespace = (line: string): [string, string] | null => {
  const index = line.search(/\s/);
  if (index < 0) {
    return null;
  }

  const key = line.slice(0, index);
  const value = line.slice(index).trim();
  return [key, value];
};

interface ParsedSshConfigHost {
  host: string;
  user?: string;
  port?: number;
}

const expandHostTokens = (value: string): string[] => {
  return value
    .split(/\s+/g)
    .map((token) => token.trim())
    .filter(
      (host) =>
        host.length > 0 &&
        host !== '*' &&
        !host.startsWith('!') &&
        !host.includes('*') &&
        !host.includes('?'),
    );
};

const parseSshConfigHostEntries = (raw: string): ParsedSshConfigHost[] => {
  const rows = new Map<string, ParsedSshConfigHost>();
  let activeHosts: string[] = [];

  const commitActiveHosts = () => {
    for (const host of activeHosts) {
      if (!rows.has(host)) {
        rows.set(host, { host });
      }
    }
  };

  for (const line of raw.split('\n')) {
    const stripped = stripInlineComment(line).trim();
    if (!stripped) {
      continue;
    }

    const parts = splitOnceWhitespace(stripped);
    if (!parts) {
      continue;
    }

    const [rawKey, value] = parts;
    const key = rawKey.toLowerCase();

    if (key === 'host') {
      commitActiveHosts();
      activeHosts = expandHostTokens(value);
      commitActiveHosts();
      continue;
    }

    if (activeHosts.length === 0) {
      continue;
    }

    if (key === 'user') {
      const user = value.trim();
      if (!user) {
        continue;
      }

      for (const host of activeHosts) {
        const row = rows.get(host);
        if (!row || row.user) {
          continue;
        }

        row.user = user;
      }
      continue;
    }

    if (key === 'port') {
      const parsed = Number.parseInt(value.trim(), 10);
      if (!Number.isFinite(parsed) || parsed <= 0 || parsed > 65_535) {
        continue;
      }

      for (const host of activeHosts) {
        const row = rows.get(host);
        if (!row || row.port) {
          continue;
        }

        row.port = parsed;
      }
    }
  }

  return [...rows.values()];
};

export const parseSshConfigHosts = (raw: string): string[] => {
  return parseSshConfigHostEntries(raw).map((entry) => entry.host);
};

const readSshConfigHosts = (): ParsedSshConfigHost[] => {
  try {
    const configPath = join(homedir(), '.ssh', 'config');
    const raw = readFileSync(configPath, 'utf8');
    return parseSshConfigHostEntries(raw);
  } catch {
    return [];
  }
};

const sanitizeSessionToken = (value: string): string => {
  return value
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '')
    .slice(0, 48);
};

const resolveSessionName = (host: string, forwards: PortForwardSpec[], sessionName?: string): string => {
  const normalized = sessionName?.trim();
  if (normalized) {
    return normalized;
  }

  const hostToken = sanitizeSessionToken(host) || 'host';
  const firstForward = forwards[0];
  const portToken = firstForward
    ? `${firstForward.localPort}-${firstForward.remotePort}`
    : Math.floor(Math.random() * 100_000).toString(36);
  return `dark-ssh-${hostToken}-${portToken}`;
};

const findConfiguredHost = (hostKey: string) => {
  return getConfig().ssh.hosts.find((candidate) => candidate.host === hostKey);
};

export const listSshHosts = (): SshHostInfo[] => {
  const config = getConfig();
  const rows: SshHostInfo[] = [];
  const seen = new Set<string>();

  for (const host of config.ssh.hosts) {
    const key = host.host.trim();
    if (!key || seen.has(key)) {
      continue;
    }

    seen.add(key);
    rows.push({
      key,
      host: key,
      source: 'config',
      label: host.name ? `${host.name} (${key})` : `${key} (config)`,
      user: host.user,
      port: host.port,
      defaultPath: host.defaultPath,
    });
  }

  for (const host of readSshConfigHosts()) {
    if (seen.has(host.host)) {
      continue;
    }

    seen.add(host.host);
    rows.push({
      key: host.host,
      host: host.host,
      source: 'ssh_config',
      label: `${host.host} (ssh config)`,
      user: host.user,
      port: host.port,
    });
  }

  rows.sort((left, right) => left.key.localeCompare(right.key));
  return rows;
};

export const listSshPortForwardPresets = (): SshPortForwardPreset[] => {
  return getConfig().ssh.portForwards.map((preset) => ({
    name: preset.name,
    host: preset.host,
    localPort: preset.localPort,
    remotePort: preset.remotePort,
    remoteHost: preset.remoteHost,
    description: preset.description,
  }));
};

export const buildSshInvocation = (
  hostKey: string,
  extraArgs: readonly string[] = [],
): SshCommandInvocation => {
  const config = getConfig();
  const normalizedHostKey = hostKey.trim();
  if (!normalizedHostKey) {
    throw new Error('SSH // Host // Host is required');
  }

  const configured = findConfiguredHost(normalizedHostKey);
  const destination =
    configured?.user && !normalizedHostKey.includes('@')
      ? `${configured.user}@${normalizedHostKey}`
      : normalizedHostKey;

  const args: string[] = [
    ...config.ssh.defaultSshArgs,
    ...(configured?.sshArgs ?? []),
  ];

  if (configured?.port) {
    args.push('-p', String(configured.port));
  }

  args.push(...extraArgs);

  return {
    hostKey: normalizedHostKey,
    destination,
    args,
  };
};

export const startSshPortForward = (input: StartSshPortForwardInput): StartSshPortForwardResult => {
  if (input.forwards.length === 0) {
    throw new Error('SSH // Port Forward // At least one forward mapping is required');
  }

  const sessionName = resolveSessionName(input.host, input.forwards, input.sessionName);
  const invocation = buildSshInvocation(input.host, [
    '-o',
    'BatchMode=yes',
    '-o',
    'ExitOnForwardFailure=yes',
    ...(input.extraSshArgs ?? []),
  ]);

  const forwardSpecs = input.forwards.map((forward) => {
    const remoteHost = forward.remoteHost?.trim() || '127.0.0.1';
    return `${forward.localPort}:${remoteHost}:${forward.remotePort}`;
  });

  const sshCommandParts: string[] = ['ssh', ...invocation.args, '-N'];
  for (const spec of forwardSpecs) {
    sshCommandParts.push('-L', spec);
  }
  sshCommandParts.push(invocation.destination);
  const sshCommand = shellJoin(sshCommandParts);

  ensureTmuxAvailable();

  if (tmuxSessionExists(sessionName)) {
    return {
      sessionName,
      host: invocation.hostKey,
      command: sshCommand,
      forwardSpecs,
      alreadyRunning: true,
    };
  }

  const launch = runTmux(['new-session', '-d', '-s', sessionName, '/bin/sh', '-lc', sshCommand]);
  if (!launch.ok) {
    throw new Error(
      `SSH // Port Forward // Failed to launch tmux session (session=${sessionName},stderr=${launch.stderr || '<empty>'},exit=${launch.exitCode ?? 'null'})`,
    );
  }

  return {
    sessionName,
    host: invocation.hostKey,
    command: sshCommand,
    forwardSpecs,
    alreadyRunning: false,
  };
};

const parseTmuxSessionList = (raw: string): TmuxSessionInfo[] => {
  const rows: TmuxSessionInfo[] = [];
  for (const line of raw.split('\n')) {
    const trimmed = line.trim();
    if (!trimmed) {
      continue;
    }

    const [name, attachedRaw, windowsRaw] = trimmed.split('\t');
    if (!name) {
      continue;
    }

    const panes = runTmux(['list-panes', '-t', name, '-F', '#{pane_current_command}']);
    const currentCommand = panes.ok
      ? panes.stdout
          .split('\n')
          .map((entry) => entry.trim())
          .find((entry) => entry.length > 0) ?? '-'
      : '-';

    rows.push({
      name,
      attached: attachedRaw === '1',
      windows: Number.parseInt(windowsRaw ?? '0', 10) || 0,
      currentCommand,
    });
  }

  rows.sort((left, right) => left.name.localeCompare(right.name));
  return rows;
};

export const listTmuxSessions = (): TmuxSessionInfo[] => {
  const version = runTmux(['-V']);
  if (!version.ok) {
    return [];
  }

  const list = runTmux(['list-sessions', '-F', '#{session_name}\t#{session_attached}\t#{session_windows}']);
  if (!list.ok) {
    return [];
  }

  return parseTmuxSessionList(list.stdout);
};

export const listActiveSshForwardSessions = (): TmuxSessionInfo[] => {
  return listTmuxSessions().filter((session) => {
    const byName = session.name.startsWith('dark-ssh-');
    const byCommand = session.currentCommand === 'ssh';
    return byName || byCommand;
  });
};
