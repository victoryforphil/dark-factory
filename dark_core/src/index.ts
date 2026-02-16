import { DEFAULT_CONFIG_PATH, createDefaultConfig, loadConfig, redactConfig, writeConfig } from './config';
import { toTomlString } from './config/lib/toml';
import Log from './utils/logging';

type PrintFormat = 'pretty' | 'json' | 'toml';

const readOptionValue = (args: string[], optionName: string): string | undefined => {
  const optionWithEquals = `${optionName}=`;

  for (let index = 0; index < args.length; index += 1) {
    const arg = args[index]!;

    if (arg === optionName) {
      return args[index + 1];
    }

    if (arg.startsWith(optionWithEquals)) {
      return arg.slice(optionWithEquals.length);
    }
  }

  return undefined;
};

const resolvePrintFormat = (args: string[]): PrintFormat => {
  const hasJson = args.includes('--json');
  const hasToml = args.includes('--toml');

  if (hasJson && hasToml) {
    throw new Error('Config // CLI // Invalid flags (--json and --toml are mutually exclusive)');
  }

  if (hasJson) {
    return 'json';
  }

  if (hasToml) {
    return 'toml';
  }

  return 'pretty';
};

const printUsage = (): void => {
  console.info('Usage:');
  console.info('  bun run src/index.ts');
  console.info('  bun run src/index.ts config export [--path <path>]');
  console.info('  bun run src/index.ts config print [--path <path>] [--json|--toml]');
};

const runServer = async (): Promise<void> => {
  const { buildApp } = await import('./app');
  const config = loadConfig();
  const app = buildApp();

  app.listen({
    hostname: config.server.listenHost,
    port: config.server.listenPort,
  });

  Log.info(
    `Core // HTTP // Listening (env=${config.env},host=${config.server.listenHost},port=${config.server.listenPort})`,
  );
};

const runConfigExport = (args: string[]): void => {
  const path = readOptionValue(args, '--path') ?? DEFAULT_CONFIG_PATH;
  const defaults = createDefaultConfig();

  writeConfig(path, defaults);
  Log.info(`Core // Config CLI // Exported defaults (path=${path})`);
};

const runConfigPrint = (args: string[]): void => {
  const path = readOptionValue(args, '--path');
  const format = resolvePrintFormat(args);
  const config = redactConfig(loadConfig(path ? { path } : {}));

  if (format === 'json') {
    console.info(JSON.stringify(config));
    return;
  }

  if (format === 'toml') {
    console.info(toTomlString(config));
    return;
  }

  console.info(JSON.stringify(config, null, 2));
};

const runCli = async (): Promise<void> => {
  const args = Bun.argv.slice(2);

  if (args.length === 0) {
    await runServer();
    return;
  }

  const [command, subcommand, ...rest] = args;

  if (command !== 'config') {
    printUsage();
    throw new Error(`Config // CLI // Unknown command (command=${command})`);
  }

  if (subcommand === 'export') {
    runConfigExport(rest);
    return;
  }

  if (subcommand === 'print') {
    runConfigPrint(rest);
    return;
  }

  printUsage();
  throw new Error(`Config // CLI // Unknown config subcommand (subcommand=${subcommand ?? 'none'})`);
};

runCli().catch((error) => {
  const message = error instanceof Error ? error.message : String(error);
  Log.error(`Core // CLI // Failed (reason=${message})`);
  process.exit(1);
});
