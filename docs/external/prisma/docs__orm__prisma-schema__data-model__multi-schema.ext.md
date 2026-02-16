----
## External Docs Snapshot // prisma

- Captured: 2026-02-16T05:57:22.190Z
- Source root: https://www.prisma.io/docs/orm/prisma-schema
- Source page: /docs/orm/prisma-schema/data-model/multi-schema
- Keywords: prisma, prisma schema, orm, docs, docs, orm, prisma schema, data model, multi schema
- Summary: Overview[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/multi-schema#overview "Direct link to Overview")
----

Source: https://www.prisma.io/docs/orm/prisma-schema/data-model/multi-schema

Multi-schema
------------

Overview[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/multi-schema#overview "Direct link to Overview")
-------------------------------------------------------------------------------------------------------------------

PostgreSQL, CockroachDB, and SQL Server allow you to organize database tables into named groups. These groups are known as _schemas_ and act as a _namespace_ for logically grouping tables (e.g. to avoid name collisions or to have clearer domain separation) and let you define foreign key constraints across them. To avoid ambiguity, this page will refer to these namespaces as _database schemas_.

This page explains how to:

*   include multiple database schemas in your Prisma schema
*   apply your schema changes to your database with Prisma Migrate
*   introspect an existing database with multiple database schemas
*   query across multiple database schemas with Prisma Client

note

Multi-schema feature is only supported for PostgreSQL, CockroachDB, and SQL Server. It is not available for SQLite and MySQL because these databases don't have the same concept of _schemas as namespaces_.

How to include multiple database schemas in your Prisma schema[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/multi-schema#how-to-include-multiple-database-schemas-in-your-prisma-schema "Direct link to How to include multiple database schemas in your Prisma schema")
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

To use multiple database schemas in your Prisma schema file, add the names of your database schemas to an array in the `schemas` field, in the `datasource` block. The following example adds a `"base"` and a `"shop"` schema:

schema.prisma

`generator client {  provider        = "prisma-client"  output          = "./generated"}datasource db {  provider = "postgresql"  schemas  = ["base", "shop"]}`

You do not need to change your connection string. The `schema` value of your connection string is the default database schema that Prisma Client connects to and uses for raw queries. All other Prisma Client queries use the schema of the model or enum that you are querying.

To designate that a model or enum belongs to a specific database schema, add the `@@schema` attribute with the name of the database schema as a parameter. In the following example, the `User` model is part of the `"base"` schema, and the `Order` model and `Size` enum are part of the `"shop"` schema:

schema.prisma

`model User {  id     Int     @id  orders Order[]  @@schema("base")}model Order {  id      Int  @id  user    User @relation(fields: [userId], references: [id])  userId  Int  @@schema("shop")}enum Size {  Small  Medium  Large  @@schema("shop")}`

### Tables with the same name in different database schemas[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/multi-schema#tables-with-the-same-name-in-different-database-schemas "Direct link to Tables with the same name in different database schemas")

If you have tables with the same name in different database schemas, you will need to map the table names to unique model names in your Prisma schema. This avoids name conflicts when you query models in Prisma Client.

For example, consider a situation where the `Config` table in the `base` database schema has the same name as the `Config` table in the `users` database schema. To avoid name conflicts, give the models in your Prisma schema unique names (`BaseConfig` and `UserConfig`) and use the `@@map` attribute to map each model to the corresponding table name:

schema.prisma

`model BaseConfig {  id Int @id  @@map("Config")  @@schema("base")}model UserConfig {  id Int @id  @@map("Config")  @@schema("users")}`

How to apply your schema changes with Prisma Migrate[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/multi-schema#how-to-apply-your-schema-changes-with-prisma-migrate "Direct link to How to apply your schema changes with Prisma Migrate")
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

You can use Prisma Migrate (or `prisma db push`) to apply changes to a Prisma schema with multiple database schemas.

As an example, add a `Profile` model to the `base` schema above:

schema.prisma

`model User {  id      Int      @id  orders  Order[]  profile Profile?  @@schema("base")}model Profile {  id     Int    @id @default(autoincrement())  bio    String  user   User   @relation(fields: [userId], references: [id])  userId Int    @unique  @@schema("base")}model Order {  id      Int  @id  user    User @relation(fields: [userId], references: [id])  userId  Int  @@schema("shop")}enum Size {  Small  Medium  Large  @@schema("shop")}`

You can then apply this schema change to your database. For example, you can use `migrate dev` to create and apply your schema changes as a migration:

`npx prisma migrate dev --name add_profile`

Note that if you move a model or enum from one schema to another, Prisma ORM deletes the model or enum from the source schema and creates a new one in the target schema.

How to introspect an existing database with multiple database schemas[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/multi-schema#how-to-introspect-an-existing-database-with-multiple-database-schemas "Direct link to How to introspect an existing database with multiple database schemas")
----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

You can introspect an existing database that has multiple database schemas in the same way that you introspect a database that has a single database schema, using `prisma db pull`:

`npx prisma db pull`

This updates your Prisma schema to match the current state of the database.

If you have tables with the same name in different database schemas, Prisma ORM shows a validation error pointing out the conflict. To fix this, [rename the introspected models with the `@map` attribute](http://www.prisma.io/docs/orm/prisma-schema/data-model/multi-schema#tables-with-the-same-name-in-different-database-schemas).

How to query across multiple database schemas with Prisma Client[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/multi-schema#how-to-query-across-multiple-database-schemas-with-prisma-client "Direct link to How to query across multiple database schemas with Prisma Client")
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

You can query models in multiple database schemas without any change to your Prisma Client query syntax. For example, the following query finds all orders for a given user, using the Prisma schema above:

`const orders = await prisma.order.findMany({  where: {    user: {      id: 42,    },  },})`

### Externally managed tables[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/multi-schema#externally-managed-tables "Direct link to Externally managed tables")

Sometimes, you might not want Prisma ORM to manage specific tables, such as ones handled by another team or service (e.g., Auth0 or Clerk tables). In such cases, you can mark these as **externally managed tables** using the `tables.external` configuration option in your [Prisma Config file](https://www.prisma.io/docs/orm/reference/prisma-config-reference#tablesexternal-and-enumsexternal). Learn more about [externally managed tables](https://www.prisma.io/docs/orm/prisma-schema/data-model/externally-managed-tables).

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery, scoped to prisma-schema docs subtree.
- Conversion path: r.jina.ai markdown proxy.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
