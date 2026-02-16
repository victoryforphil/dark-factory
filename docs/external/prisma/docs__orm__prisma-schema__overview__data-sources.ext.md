----
## External Docs Snapshot // prisma

- Captured: 2026-02-16T05:57:22.190Z
- Source root: https://www.prisma.io/docs/orm/prisma-schema
- Source page: /docs/orm/prisma-schema/overview/data-sources
- Keywords: prisma, prisma schema, orm, docs, docs, orm, prisma schema, overview, data sources
- Summary: A data source determines how Prisma ORM connects to your database, and is represented by the [`datasource`](/docs/orm/reference/prisma-schema-reference#datasource) block in the Prisma schema. The following data source uses the `postgresq...
----

Source: https://www.prisma.io/docs/orm/prisma-schema/overview/data-sources

- [ORM](/docs/orm)
- [Prisma Schema](/docs/orm/prisma-schema)
- [Overview](/docs/orm/prisma-schema/overview)

# Data sources

A data source determines how Prisma ORM connects to your database, and is represented by the [`datasource`](/docs/orm/reference/prisma-schema-reference#datasource) block in the Prisma schema. The following data source uses the `postgresql` provider and includes a connection URL:

note

As of Prisma ORM v7, the `url`, `directUrl`, and `shadowDatabaseUrl` fields in the Prisma schema `datasource` block are deprecated. Configure these fields in [Prisma Config](/docs/orm/reference/prisma-config-reference) instead.

```
datasource db {  provider = "postgresql"  url      = "postgresql://johndoe:mypassword@localhost:5432/mydb?schema=public"}
```

A Prisma schema can only have one data source. However, you can:

- [Programmatically override a data source `url` when creating your `PrismaClient`](/docs/orm/reference/prisma-client-reference#programmatically-override-a-datasource-url)

- [Specify a different URL for Prisma Migrate's shadow database if you are working with cloud-hosted development databases](/docs/orm/prisma-migrate/understanding-prisma-migrate/shadow-database#cloud-hosted-shadow-databases-must-be-created-manually)

Note: Multiple provider support was removed in 2.22.0. Please see [Deprecation of provider array notation](https://github.com/prisma/prisma/issues/3834) for more information.

## Securing database connections[​](#securing-database-connections)

Some data source `provider`s allow you to configure your connection with SSL/TLS, and provide parameters for the `url` to specify the location of certificates.

- [Configuring an SSL connection with PostgreSQL](/docs/orm/overview/databases/postgresql#configuring-an-ssl-connection)

- [Configuring an SSL connection with MySQL](/docs/orm/overview/databases/mysql#configuring-an-ssl-connection)

- [Configure a TLS connection with Microsoft SQL Server](/docs/orm/overview/databases/sql-server#connection-details)

Prisma ORM resolves SSL certificates relative to the `./prisma` directory. If your certificate files are located outside that directory, e.g. your project root directory, use relative paths for certificates:

note

When you're using a [multi-file Prisma schema](/docs/orm/prisma-schema/overview/location#multi-file-prisma-schema), Prisma ORM resolves SSL certificates relative to the `./prisma/schema` directory.

```
datasource db {  provider = "postgresql"  url      = "postgresql://johndoe:mypassword@localhost:5432/mydb?schema=public&sslmode=require&sslcert=../server-ca.pem&sslidentity=../client-identity.p12&sslpassword="}
```

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery, scoped to prisma-schema docs subtree.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
