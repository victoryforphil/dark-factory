----
## External Docs Snapshot // prisma

- Captured: 2026-02-16T05:57:22.190Z
- Source root: https://www.prisma.io/docs/orm/prisma-schema
- Source page: /docs/orm/prisma-schema/postgresql-extensions
- Keywords: prisma, prisma schema, orm, docs, docs, orm, prisma schema, postgresql extensions
- Summary: This page is about [PostgreSQL extensions](https://www.postgresql.org/docs/current/external-extensions.html) and explains how to use them with Prisma ORM.
----

Source: https://www.prisma.io/docs/orm/prisma-schema/postgresql-extensions

- [ORM](/docs/orm)
- [Prisma Schema](/docs/orm/prisma-schema)

# PostgreSQL extensions

This page is about [PostgreSQL extensions](https://www.postgresql.org/docs/current/external-extensions.html) and explains how to use them with Prisma ORM.

warning

Between Prisma ORM v4.5.0 and v6.16.0, you could enable extensions in the Prisma schema via the `postgresqlExtensions` preview feature flag. This feature flag has been deprecated in v6.16.0 and the recommended approach for using PostgreSQL extensions now is to install them via [customized migrations](/docs/orm/prisma-migrate/workflows/customizing-migrations).

## What are PostgreSQL extensions?[​](#what-are-postgresql-extensions)

PostgreSQL allows you to extend your database functionality by installing and activating packages known as extensions. For example, the `citext` extension adds a case-insensitive string data type. Some extensions, such as `citext`, are supplied directly by PostgreSQL, while other extensions are developed externally. For more information on extensions, see [the PostgreSQL documentation](https://www.postgresql.org/docs/current/sql-createextension.html).

To use an extension, it must first be installed on the local file system of your database server. You then need to activate the extension, which runs a script file that adds the new functionality.

## Using a PostgreSQL extension with Prisma ORM[​](#using-a-postgresql-extension-with-prisma-orm)

Let's walk through an example of installing the `citext` extension.

### 1. Create an empty migration[​](#1-create-an-empty-migration)

Run the following command to create an empty migration that you can [customize](/docs/orm/prisma-migrate/workflows/customizing-migrations):

```
npx prisma migrate dev --create-only
```

### 2. Add a SQL statement to install the extension[​](#2-add-a-sql-statement-to-install-the-extension)

In the new migration file that was created in the `migrations` directory, add the following statement:

```
CREATE EXTENSION IF NOT EXISTS citext;
```

### 3. Deploy the migration[​](#3-deploy-the-migration)

Run the following command to deploy the migration and apply to your database:

```
npx prisma migrate deploy
```

### 4. Use the extension[​](#4-use-the-extension)

You can now use the extension in your queries with Prisma Client. If the extension has special data types that currently can't be natively represented in the Prisma schema, you can still define fields of that type on your models using the [`Unsupported`](/docs/orm/prisma-schema/data-model/models#unsupported-types) fallback type.

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery, scoped to prisma-schema docs subtree.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
