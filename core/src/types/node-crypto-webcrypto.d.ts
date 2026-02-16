declare module "node:crypto" {
  namespace webcrypto {
    type BufferSource = NodeJS.BufferSource;
  }
}
