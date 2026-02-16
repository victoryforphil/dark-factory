#!/usr/bin/env bun

export type CommandStep = {
  name: string;
  command: string;
  args?: string[];
  cwd?: string;
};

export async function runCommandSteps(steps: CommandStep[]): Promise<void> {
  for (const [index, step] of steps.entries()) {
    const stepNumber = index + 1;
    console.log(`Install // Step // ${stepNumber}/${steps.length} ${step.name}`);

    const process = Bun.spawn([step.command, ...(step.args ?? [])], {
      cwd: step.cwd,
      stdin: "inherit",
      stdout: "inherit",
      stderr: "inherit",
    });

    const exitCode = await process.exited;
    if (exitCode !== 0) {
      throw new Error(
        `Install // Step // Failed (${step.name},exit=${exitCode})`,
      );
    }
  }
}
