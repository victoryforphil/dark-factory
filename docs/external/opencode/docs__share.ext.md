----
## External Docs Snapshot // opencode

- Captured: 2026-02-16T04:13:51.889Z
- Source root: https://opencode.ai/docs
- Source page: /docs/share
- Keywords: opencode, docs, ai coding assistant, cli, share
- Summary: Share your OpenCode conversations.
----

Source: https://opencode.ai/docs/share

# Share

Share your OpenCode conversations.

OpenCode’s share feature allows you to create public links to your OpenCode conversations, so you can collaborate with teammates or get help from others.

Note

Shared conversations are publicly accessible to anyone with the link.

## [How it works](#how-it-works)

When you share a conversation, OpenCode:

- Creates a unique public URL for your session

- Syncs your conversation history to our servers

- Makes the conversation accessible via the shareable link — `opncd.ai/s/&#x3C;share-id>`

## [Sharing](#sharing)

OpenCode supports three sharing modes that control how conversations are shared:

### [Manual (default)](#manual-default)

By default, OpenCode uses manual sharing mode. Sessions are not shared automatically, but you can manually share them using the `/share` command:

- ``` /share ``` This will generate a unique URL that’ll be copied to your clipboard. To explicitly set manual mode in your [config file](/docs/config): opencode.json ``` { "$schema": "https://opncd.ai/config.json", "share": "manual"} ``` ### [Auto-share](#auto-share) You can enable automatic sharing for all new conversations by setting the `share` option to `"auto"` in your [config file](/docs/config): opencode.json ``` { "$schema": "https://opncd.ai/config.json", "share": "auto"} ``` With auto-share enabled, every new conversation will automatically be shared and a link will be generated. ### [Disabled](#disabled) You can disable sharing entirely by setting the `share` option to `"disabled"` in your [config file](/docs/config): opencode.json ``` { "$schema": "https://opncd.ai/config.json", "share": "disabled"} ``` To enforce this across your team for a given project, add it to the `opencode.json` in your project and check into Git. ## [Un-sharing](#un-sharing) To stop sharing a conversation and remove it from public access: ``` /unshare ``` This will remove the share link and delete the data related to the conversation. ## [Privacy](#privacy) There are a few things to keep in mind when sharing a conversation. ### [Data retention](#data-retention) Shared conversations remain accessible until you explicitly unshare them. This includes: Full conversation history

- All messages and responses

- Session metadata

### [Recommendations](#recommendations)

- Only share conversations that don’t contain sensitive information.

- Review conversation content before sharing.

- Unshare conversations when collaboration is complete.

- Avoid sharing conversations with proprietary code or confidential data.

- For sensitive projects, disable sharing entirely.

## [For enterprises](#for-enterprises)

For enterprise deployments, the share feature can be:

- Disabled entirely for security compliance

- Restricted to users authenticated through SSO only

- Self-hosted on your own infrastructure

[Learn more](/docs/enterprise) about using opencode in your organization.

[Edit page](https://github.com/anomalyco/opencode/edit/dev/packages/web/src/content/docs/share.mdx)[Found a bug? Open an issue](https://github.com/anomalyco/opencode/issues/new)[Join our Discord community](https://opencode.ai/discord) Select language   EnglishالعربيةBosanskiDanskDeutschEspañolFrançaisItaliano日本語한국어Norsk BokmålPolskiPortuguês (Brasil)РусскийไทยTürkçe简体中文繁體中文

&copy; [Anomaly](https://anoma.ly)

Last updated: Feb 15, 2026

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
