----
## External Docs Snapshot // prisma

- Captured: 2026-02-16T05:57:22.190Z
- Source root: https://www.prisma.io/docs/orm/prisma-schema
- Source page: /docs/orm/prisma-schema/data-model
- Keywords: prisma, prisma schema, orm, docs, docs, orm, prisma schema, data model
- Summary: Data model | Prisma Documentation
----

Source: https://www.prisma.io/docs/orm/prisma-schema/data-model

Data model | Prisma Documentation
===============

[Skip to main content](http://www.prisma.io/docs/orm/prisma-schema/data-model#__docusaurus_skipToContent_fallback)

[![Image 1: Prisma logo](http://www.prisma.io/docs/img/logo.svg)](https://www.prisma.io/)/[docs](http://www.prisma.io/docs/)

[Get Started](http://www.prisma.io/docs/getting-started)[Postgres](http://www.prisma.io/docs/postgres)[ORM](http://www.prisma.io/docs/orm)[Guides](http://www.prisma.io/docs/guides)

[More](http://www.prisma.io/docs/orm/prisma-schema/data-model#)
*   [Studio Explore and manipulate your data](http://www.prisma.io/docs/postgres/database/prisma-studio)
*   [Optimize AI-driven query analysis](http://www.prisma.io/docs/optimize)
*   [Accelerate Make your database global](http://www.prisma.io/docs/accelerate)
*   [Prisma + AI Build faster with Prisma + AI](http://www.prisma.io/docs/ai)

[](https://pris.ly/discord?utm_source=docs&utm_medium=navbar)[](https://pris.ly/github?utm_source=docs&utm_medium=navbar)[Log in](https://console.prisma.io/login?utm_source=docs&utm_medium=login)

Search

[Try Prisma Postgres. No cold starts, just hot queries.](https://pris.ly/sidebar-promo/hot-queries)
*   [ORM](http://www.prisma.io/docs/orm) 
    *   [ORM](http://www.prisma.io/docs/orm)
    *   [Getting started](http://www.prisma.io/docs/orm/getting-started) 
        *   [Quickstart](http://www.prisma.io/docs/orm/getting-started/quickstart)
        *   [Add to existing project](http://www.prisma.io/docs/orm/getting-started/add-to-existing-project)

    *   [Overview](http://www.prisma.io/docs/orm/overview) 
        *   [Introduction](http://www.prisma.io/docs/orm/overview/introduction) 
        *   [Prisma ORM in your stack](http://www.prisma.io/docs/orm/overview/prisma-in-your-stack) 
        *   [Databases](http://www.prisma.io/docs/orm/overview/databases) 
        *   [Beyond Prisma ORM](http://www.prisma.io/docs/orm/overview/beyond-prisma-orm)

    *   [Prisma Schema](http://www.prisma.io/docs/orm/prisma-schema) 
        *   [Overview](http://www.prisma.io/docs/orm/prisma-schema/overview) 
        *   [Data model](http://www.prisma.io/docs/orm/prisma-schema/data-model) 
            *   [Models](http://www.prisma.io/docs/orm/prisma-schema/data-model/models)
            *   [Relations](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations) 
            *   [Indexes](http://www.prisma.io/docs/orm/prisma-schema/data-model/indexes)
            *   [Views](http://www.prisma.io/docs/orm/prisma-schema/data-model/views)
            *   [Database mapping](http://www.prisma.io/docs/orm/prisma-schema/data-model/database-mapping)
            *   [Multi-schema](http://www.prisma.io/docs/orm/prisma-schema/data-model/multi-schema)
            *   [External tables](http://www.prisma.io/docs/orm/prisma-schema/data-model/externally-managed-tables)
            *   [Unsupported database features](http://www.prisma.io/docs/orm/prisma-schema/data-model/unsupported-database-features)
            *   [Table inheritance](http://www.prisma.io/docs/orm/prisma-schema/data-model/table-inheritance)

        *   [Introspection](http://www.prisma.io/docs/orm/prisma-schema/introspection)
        *   [PostgreSQL extensions](http://www.prisma.io/docs/orm/prisma-schema/postgresql-extensions)

    *   [Prisma Client](http://www.prisma.io/docs/orm/prisma-client) 
        *   [Setup & configuration](http://www.prisma.io/docs/orm/prisma-client/setup-and-configuration) 
        *   [Queries](http://www.prisma.io/docs/orm/prisma-client/queries) 
        *   [Write your own SQL](http://www.prisma.io/docs/orm/prisma-client/using-raw-sql) 
        *   [Fields & types](http://www.prisma.io/docs/orm/prisma-client/special-fields-and-types) 
        *   [Extensions](http://www.prisma.io/docs/orm/prisma-client/client-extensions) 
        *   [Type safety](http://www.prisma.io/docs/orm/prisma-client/type-safety) 
        *   [Testing](http://www.prisma.io/docs/orm/prisma-client/testing) 
        *   [Deployment](http://www.prisma.io/docs/orm/prisma-client/deployment) 
        *   [Observability & logging](http://www.prisma.io/docs/orm/prisma-client/observability-and-logging) 
        *   [Debugging & troubleshooting](http://www.prisma.io/docs/orm/prisma-client/debugging-and-troubleshooting) 

    *   [Prisma Migrate](http://www.prisma.io/docs/orm/prisma-migrate) 
        *   [Getting started](http://www.prisma.io/docs/orm/prisma-migrate/getting-started)
        *   [Understanding Prisma Migrate](http://www.prisma.io/docs/orm/prisma-migrate/understanding-prisma-migrate) 
        *   [Workflows](http://www.prisma.io/docs/orm/prisma-migrate/workflows) 

    *   [Tools](http://www.prisma.io/docs/orm/tools) 
        *   [Prisma CLI](http://www.prisma.io/docs/orm/tools/prisma-cli)
        *   [Prisma Studio](http://www.prisma.io/docs/orm/tools/prisma-studio)

    *   [Reference](http://www.prisma.io/docs/orm/reference) 
        *   [Prisma Client API](http://www.prisma.io/docs/orm/reference/prisma-client-reference)
        *   [Prisma Schema](http://www.prisma.io/docs/orm/reference/prisma-schema-reference)
        *   [Prisma CLI](http://www.prisma.io/docs/orm/reference/prisma-cli-reference)
        *   [Errors](http://www.prisma.io/docs/orm/reference/error-reference)
        *   [Environment variables](http://www.prisma.io/docs/orm/reference/environment-variables-reference)
        *   [errors](http://www.prisma.io/docs/orm/prisma-schema/data-model#) 
        *   [Prisma Config](http://www.prisma.io/docs/orm/reference/prisma-config-reference)
        *   [Database features matrix](http://www.prisma.io/docs/orm/reference/database-features)
        *   [Supported databases](http://www.prisma.io/docs/orm/reference/supported-databases)
        *   [Connection URLs](http://www.prisma.io/docs/orm/reference/connection-urls)
        *   [System requirements](http://www.prisma.io/docs/orm/reference/system-requirements)
        *   [Preview features](http://www.prisma.io/docs/orm/reference/preview-features) 

    *   [More](http://www.prisma.io/docs/orm/more) 
        *   [Under the hood](http://www.prisma.io/docs/orm/more/under-the-hood) 
        *   [Upgrade guides](http://www.prisma.io/docs/orm/more/upgrade-guides) 
        *   [AI tools](http://www.prisma.io/docs/orm/more/ai-tools) 
        *   [Comparing Prisma ORM](http://www.prisma.io/docs/orm/more/comparisons) 
        *   [Development environment](http://www.prisma.io/docs/orm/more/development-environment) 
        *   [Help articles](http://www.prisma.io/docs/orm/more/help-and-troubleshooting) 
        *   [ORM releases and maturity levels](http://www.prisma.io/docs/orm/more/releases)

*   [](http://www.prisma.io/docs)
*   [ORM](http://www.prisma.io/docs/orm)
*   [Prisma Schema](http://www.prisma.io/docs/orm/prisma-schema)

Data model
==========

In this section[​](http://www.prisma.io/docs/orm/prisma-schema/data-model#in-this-section "Direct link to In this section")
---------------------------------------------------------------------------------------------------------------------------

[Models ------ The data model definition part of the Prisma schema defines your application models (also called Prisma models). Models:](http://www.prisma.io/docs/orm/prisma-schema/data-model/models)[Relations ---------](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations)[Indexes ------- Prisma ORM allows configuration of database indexes, unique constraints and primary key constraints. This is in General Availability in versions 4.0.0 and later. You can enable this with the extendedIndexes Preview feature in versions 3.5.0 and later.](http://www.prisma.io/docs/orm/prisma-schema/data-model/indexes)[Views ----- Database views allow you to name and store queries. In relational databases, views are stored SQL queries that might include columns in multiple tables, or calculated values such as aggregates. In MongoDB, views are queryable objects where the contents are defined by an aggregation pipeline on other collections.](http://www.prisma.io/docs/orm/prisma-schema/data-model/views)[Database mapping ---------------- The Prisma schema includes mechanisms that allow you to define names of certain database objects. You can:](http://www.prisma.io/docs/orm/prisma-schema/data-model/database-mapping)[Multi-schema ------------ Overview](http://www.prisma.io/docs/orm/prisma-schema/data-model/multi-schema)[External tables --------------- Overview](http://www.prisma.io/docs/orm/prisma-schema/data-model/externally-managed-tables)[Unsupported database features ----------------------------- Not all database functions and features of Prisma ORM's supported databases have a Prisma Schema Language equivalent. Refer to the database features matrix for a complete list of supported features.](http://www.prisma.io/docs/orm/prisma-schema/data-model/unsupported-database-features)[Table inheritance ----------------- Overview](http://www.prisma.io/docs/orm/prisma-schema/data-model/table-inheritance)

[Previous Schema location](http://www.prisma.io/docs/orm/prisma-schema/overview/location)[Next Models](http://www.prisma.io/docs/orm/prisma-schema/data-model/models)

[![Image 2: Prisma logo](http://www.prisma.io/docs/img/logo-white.svg)](https://www.prisma.io/)

*   [](https://pris.ly/discord?utm_source=docs&utm_medium=footer)
*   [](https://pris.ly/x?utm_source=docs&utm_medium=footer)
*   [](https://pris.ly/youtube?utm_source=docs&utm_medium=footer)
*   [](https://pris.ly/whatsapp?utm_source=docs&utm_medium=footer)
*   [](https://pris.ly/github?utm_source=docs&utm_medium=footer)

Product

*   [ORM](https://www.prisma.io/orm)
*   [Studio](https://www.prisma.io/studio)
*   [Optimize](https://www.prisma.io/optimize)
*   [Accelerate](https://www.prisma.io/accelerate)
*   [Postgres](https://www.prisma.io/postgres)
*   [Pricing](https://www.prisma.io/pricing)
*   [Changelog](https://www.prisma.io/changelog)
*   [Data Platform status↗](https://www.prisma-status.com/)

Resources

*   [Docs](http://www.prisma.io/docs)
*   [Ecosystem](https://www.prisma.io/ecosystem)
*   [Playground↗](https://playground.prisma.io/)
*   [ORM Benchmarks↗](https://benchmarks.prisma.io/)
*   [Customer stories](https://www.prisma.io/showcase)
*   [Data guide](https://www.prisma.io/dataguide)

Contact us

*   [Community](https://www.prisma.io/community)
*   [Support](https://www.prisma.io/support)
*   [Enterprise](https://www.prisma.io/enterprise)
*   [Partners](https://www.prisma.io/partners)
*   [OSS Friends](https://www.prisma.io/oss-friends)

Company

*   [About](https://www.prisma.io/about)
*   [Blog](https://www.prisma.io/blog)
*   [Data DX↗](https://www.datadx.io/)
*   [Careers](https://www.prisma.io/careers)
*   [Security & Compliance](https://trust.prisma.io/)

*   Legal

[Privacy Policy](https://pris.ly/privacy)

[Terms of Service](https://pris.ly/terms)

[Service Level Agreement](https://pris.ly/sla)

[Event Code of Conduct](https://pris.ly/code-conduct)

© 2026 Prisma Data, Inc.

[![Image 3: gdpr](http://www.prisma.io/docs/img/icons/gdpr.svg)](https://trust.prisma.io/)[![Image 4: hipaa](http://www.prisma.io/docs/img/icons/hipaa.svg)](https://trust.prisma.io/)[![Image 5: iso27001](http://www.prisma.io/docs/img/icons/iso27.svg)](https://trust.prisma.io/)[![Image 6: soc](http://www.prisma.io/docs/img/icons/soc2.svg)](https://trust.prisma.io/)

![Image 7](https://t.co/1/i/adsct?bci=4&dv=UTC%26en-US%26Google%20Inc.%26Linux%20x86_64%26255%26800%26600%268%2624%26800%26600%260%26na&eci=3&event=%7B%7D&event_id=c9d046cb-4ec9-4e6e-8627-ad1134813e16&integration=gtm&p_id=Twitter&p_user_id=0&pl_id=f55fe222-0a73-4715-874d-a9a904922f59&pt=Data%20model%20%7C%20Prisma%20Documentation&tw_document_href=https%3A%2F%2Fwww.prisma.io%2Fdocs%2Form%2Fprisma-schema%2Fdata-model&tw_iframe_status=0&txn_id=o8d4i&type=javascript&version=2.3.35)![Image 8](https://analytics.twitter.com/1/i/adsct?bci=4&dv=UTC%26en-US%26Google%20Inc.%26Linux%20x86_64%26255%26800%26600%268%2624%26800%26600%260%26na&eci=3&event=%7B%7D&event_id=c9d046cb-4ec9-4e6e-8627-ad1134813e16&integration=gtm&p_id=Twitter&p_user_id=0&pl_id=f55fe222-0a73-4715-874d-a9a904922f59&pt=Data%20model%20%7C%20Prisma%20Documentation&tw_document_href=https%3A%2F%2Fwww.prisma.io%2Fdocs%2Form%2Fprisma-schema%2Fdata-model&tw_iframe_status=0&txn_id=o8d4i&type=javascript&version=2.3.35)

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery, scoped to prisma-schema docs subtree.
- Conversion path: r.jina.ai markdown proxy.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
