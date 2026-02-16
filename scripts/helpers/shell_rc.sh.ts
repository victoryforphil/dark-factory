#!/usr/bin/env bun

import { existsSync } from "node:fs";
import { dirname, join } from "node:path";

export type ShellKind = "zsh" | "bash";

export type ShellRcTarget = {
  shell: ShellKind;
  path: string;
};

export function resolveShellRcTarget(): ShellRcTarget {
  const home = resolveHomeDirectory();
  const preferredShell = detectPreferredShell();

  const preferredCandidates = rcCandidatesFor(preferredShell, home);
  for (const candidate of preferredCandidates) {
    if (existsSync(candidate)) {
      return { shell: preferredShell, path: candidate };
    }
  }

  const fallbackShell: ShellKind = preferredShell === "zsh" ? "bash" : "zsh";
  const fallbackCandidates = rcCandidatesFor(fallbackShell, home);
  for (const candidate of fallbackCandidates) {
    if (existsSync(candidate)) {
      return { shell: fallbackShell, path: candidate };
    }
  }

  return {
    shell: preferredShell,
    path: preferredCandidates[0],
  };
}

export function resolveHomeDirectory(): string {
  const home = Bun.env.HOME;
  if (!home) {
    throw new Error("Sys Install // Shell RC // HOME is not set");
  }

  return home;
}

export function rcCandidatesFor(shell: ShellKind, home: string): string[] {
  if (shell === "zsh") {
    return [join(home, ".zshrc")];
  }

  return [join(home, ".bashrc"), join(home, ".bash_profile")];
}

function detectPreferredShell(): ShellKind {
  const shellValue = Bun.env.SHELL ?? "";
  const shellName = basenameNoExt(shellValue).toLowerCase();

  if (shellName.includes("zsh")) {
    return "zsh";
  }

  return "bash";
}

function basenameNoExt(shellPath: string): string {
  const base = shellPath.trim();
  if (!base) {
    return "";
  }

  return base.slice(base.lastIndexOf("/") + 1);
}

export function shellLabel(target: ShellRcTarget): string {
  return `${target.shell} (${dirname(target.path)})`;
}
