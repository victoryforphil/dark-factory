import { describe, expect, it } from 'bun:test';

import { scanProductGitInfo, scanVariantGitInfo } from './git.scan';

describe('git scan module unit', () => {
  it('returns null for non-git local paths', async () => {
    const locator = '@local:///tmp';

    const [productInfo, variantInfo] = await Promise.all([
      scanProductGitInfo(locator),
      scanVariantGitInfo(locator),
    ]);

    expect(productInfo).toBeNull();
    expect(variantInfo).toBeNull();
  });

  it('returns null for non-local locators', async () => {
    const [productInfo, variantInfo] = await Promise.all([
      scanProductGitInfo('https://example.com/demo.git'),
      scanVariantGitInfo('https://example.com/demo.git'),
    ]);

    expect(productInfo).toBeNull();
    expect(variantInfo).toBeNull();
  });
});
