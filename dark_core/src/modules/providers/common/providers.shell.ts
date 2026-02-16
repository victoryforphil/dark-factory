export const toShellArgument = (value: string): string => {
  return `'${value.replace(/'/g, `'\\''`)}'`;
};
