----
## External Docs Snapshot // prisma

- Captured: 2026-02-16T05:57:22.190Z
- Source root: https://www.prisma.io/docs/orm/prisma-schema
- Source page: /docs/orm/prisma-schema/data-model/externally-managed-tables
- Keywords: prisma, prisma schema, orm, docs, docs, orm, prisma schema, data model, externally managed tables
- Summary: Overview[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/externally-managed-tables#overview "Direct link to Overview")
----

Source: https://www.prisma.io/docs/orm/prisma-schema/data-model/externally-managed-tables

External tables
---------------

Overview[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/externally-managed-tables#overview "Direct link to Overview")
--------------------------------------------------------------------------------------------------------------------------------

_Externally managed tables_ (or _external tables_ for short) in Prisma ORM are tables that can be **queried via Prisma Client** but are **ignored by Prisma Migrate**.

Sometimes, you might not want Prisma ORM to manage specific tables—such as ones handled by another team or service.

Some concrete use cases for this are:

*   auth services like Clerk or Auth0 that manage specific tables with user and session data
*   storage services like Supabase Storage with tables for storing metadata about buckets and objects
*   a microservice-based organization where specific teams own specific tables in the database

There may be many other scenarios based on custom organizational constraints or preferences where you may not want Prisma ORM to manage specific tables.

warning

Externally managed tables are currently in [Preview](https://www.prisma.io/docs/orm/more/releases#preview).

note

Externally managed tables are frequently used in combination with [multi-schema](https://www.prisma.io/docs/orm/prisma-schema/data-model/multi-schema) database setups. However, this is not a hard requirement. You can have only a single schema in your database and also declare externally managed tables within it.

warning

Prisma ORM will not verify that the structure of the tables in the database and the structures of the Prisma models actually match. On the one hand, it requires the developer to be thorough when updating the Prisma schema (the safest way to do it is by using `prisma db pull`). On the other hand, this flexibility enables you to represent only part of the underlying table in the database (and e.g. not expose _all_ its columns).

Workflow[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/externally-managed-tables#workflow "Direct link to Workflow")
--------------------------------------------------------------------------------------------------------------------------------

If you want to use external tables, here's the main workflow:

1.   Declare the name of the external tables in your [Prisma Config file](https://www.prisma.io/docs/orm/reference/prisma-config-reference)
2.   Update your Prisma schema (e.g. via `npx prisma db pull`)
3.   Re-generate Prisma Client with `npx prisma generate`
4.   You can now query the external table using Prisma Client but it will be ignored by Prisma Migrate
5.   When the table gets changed (by whoever owns it): 
    1.   Re-introspect your database using `npx prisma db pull` or manually update the models in your prisma file
    2.   Re-generate Prisma Client with `npx prisma generate`

Prisma Config syntax[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/externally-managed-tables#prisma-config-syntax "Direct link to Prisma Config syntax")
--------------------------------------------------------------------------------------------------------------------------------------------------------------------

You can specify externally managed tables in your [Prisma Config](https://www.prisma.io/docs/orm/reference/prisma-config-reference) file via the `tables.external` property:

prisma.config.ts

`import 'dotenv/config'import { defineConfig, env } from 'prisma/config'export default defineConfig({  schema: 'prisma/schema.prisma',  migrations: {    path: 'prisma/migrations',  },  datasource: {    url: env('DATABASE_URL'),  },  // required when using unstable features  experimental: {    externalTables: true  },  // declare the `users` table and `role` enum as external  tables: {    external: [      "public.users",    ]  },  enums: {    external: [      "public.role",    ]  },})`

*   Analogous to tables, you can also have externally managed _enums_.
*   On PostgreSQL and SQL Server you have to specify the fully qualified table/enum name including the schema name. For example: `public.products` or `auth.users`.
*   On MySQL and SQLite, you only have to specify the table name.

Relationships[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/externally-managed-tables#relationships "Direct link to Relationships")
-----------------------------------------------------------------------------------------------------------------------------------------------

Prisma can create and update relationships from tables it manages to externally managed tables.

However, for this Prisma needs to be aware of the structure of those externally managed tables during migration creation. You can provide a SQL script that Prisma will run on its [shadow database](https://www.prisma.io/docs/orm/prisma-migrate/understanding-prisma-migrate/shadow-database) ahead of all migrations to emulate the external tables and enums during migration creation.

The created placeholder table does not need to have the full structure of the actual table but primary keys need to be present.

If the external table is not referenced by any managed table—that is no managed table contains a foreign key constraint on the external table—you do NOT need to provide any SQL for it in `migrations.initShadowDb`.

prisma.config.ts

`import 'dotenv/config'import { defineConfig, env } from 'prisma/config'export default defineConfig({  schema: 'prisma/schema.prisma',  datasource: {    url: env('DATABASE_URL'),  },  // required when using unstable features  experimental: {    externalTables: true  },  // declare a `users` table  tables: {    external: [      "public.users",    ]  },  migrations: {    path: 'prisma/migrations',    // setup the users table for the shadow database    initShadowDb: `      CREATE TABLE public.users (id SERIAL PRIMARY KEY);    `  },})`

Relationships from an external table to a managed table, where the external table contains the foreign key constraint on the managed table, are **NOT** managed by Prisma as that would modify the external table.

Example[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/externally-managed-tables#example "Direct link to Example")
-----------------------------------------------------------------------------------------------------------------------------

Assume you have the following Prisma schema which only contains the `posts` table:

`generator client {  provider = "prisma-client"  output   = "./generated"  // ...}datasource db {  provider = "postgresql"  // ...}model posts {  id          Int       @id @default(autoincrement())  created_at  DateTime  @default(now())  title       String  content     String?}`

You have created that `posts` table already via a prior migration. You now also have a `users` table and `role` enum in your database which you want to treat as externally managed.

So the tables in your PostgreSQL database in the default `public` schema look like this:

`-- Enum used by users tableCREATE TYPE role AS ENUM ('customer', 'support', 'admin');-- Users tableCREATE TABLE users (  id SERIAL PRIMARY KEY,  username VARCHAR(50) NOT NULL UNIQUE,  email VARCHAR(100) NOT NULL UNIQUE,  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,  role role);-- Posts tableCREATE TABLE posts (  id SERIAL PRIMARY KEY,  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,  title VARCHAR(200) NOT NULL,  content TEXT);`

### 1. Declaring externally managed tables in Prisma Config[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/externally-managed-tables#1-declaring-externally-managed-tables-in-prisma-config "Direct link to 1. Declaring externally managed tables in Prisma Config")

Enable use of externally managed tables via the `tables.external` property:

prisma.config.ts

`import 'dotenv/config'import { defineConfig, env } from 'prisma/config'export default defineConfig({  schema: 'prisma/schema.prisma',  migrations: {    path: 'prisma/migrations',  },  datasource: {    url: env('DATABASE_URL'),  },  experimental: {    externalTables: true  },  // declare the `users` table and `role` enum as external  tables: {    external: [      "public.users",    ]  },  enums: {    external: [      "public.role",    ]  },})`

### 2. Update the Prisma schema[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/externally-managed-tables#2-update-the-prisma-schema "Direct link to 2. Update the Prisma schema")

Next, you need to update your Prisma schema. You can do this either:

*   by manually creating the models
*   or by using [introspection](https://www.prisma.io/docs/orm/prisma-schema/introspection):

`npx prisma db pull`

The `users` table is now in your Prisma schema:

`model posts {  id         Int       @id @default(autoincrement())  created_at DateTime? @default(now()) @db.Timestamp(6)  title      String    @db.VarChar(200)  content    String?}model users {  id         Int       @id @default(autoincrement())  username   String    @unique @db.VarChar(50)  email      String    @unique @db.VarChar(100)  created_at DateTime? @default(now()) @db.Timestamp(6)  role       role}enum role {  customer  support  admin}`

### 3. Re-generate Prisma Client[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/externally-managed-tables#3-re-generate-prisma-client "Direct link to 3. Re-generate Prisma Client")

In order to be able to query the `users` table, you need to re-generate Prisma Client:

`npx prisma generate`

### 4. Query the `users` table using Prisma Client[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/externally-managed-tables#4-query-the-users-table-using-prisma-client "Direct link to 4-query-the-users-table-using-prisma-client")

You can now query the external `users` table with Prisma Client:

`await prisma.users.findMany()`

### 5. Add a relationship[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/externally-managed-tables#5-add-a-relationship "Direct link to 5. Add a relationship")

Let's say you now want to add an author relationship from `posts` onto `users`.

First update your Prisma schema.

`model posts {  id         Int       @id @default(autoincrement())  created_at DateTime? @default(now()) @db.Timestamp(6)  title      String    @db.VarChar(200)  content    String?  author     users @relation(fields: [author_id], references: [id])  author_id  Int}model users {  id         Int       @id @default(autoincrement())  username   String    @unique @db.VarChar(50)  email      String    @unique @db.VarChar(100)  created_at DateTime? @default(now()) @db.Timestamp(6)  role       role  posts      posts[]}enum role {  customer  support  admin}`

Then add a `migrations.initShadowDb` script so Prisma knows about the `users` table during migrations.

prisma.config.ts

`import 'dotenv/config'import { defineConfig, env } from 'prisma/config'export default defineConfig({  schema: 'prisma/schema.prisma',  datasource: {    url: env('DATABASE_URL'),  },  experimental: {    externalTables: true  },  tables: {    external: [      "public.users",    ]  },  migrations: {    path: 'prisma/migrations',    // setup the users table for the shadow database    initShadowDb: `      CREATE TABLE public.users (id SERIAL PRIMARY KEY);    `  },})`

Now you can run `prisma migrate dev` command.

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery, scoped to prisma-schema docs subtree.
- Conversion path: r.jina.ai markdown proxy.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
