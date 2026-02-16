----
## External Docs Snapshot // prisma

- Captured: 2026-02-16T05:57:22.190Z
- Source root: https://www.prisma.io/docs/orm/prisma-schema
- Source page: /docs/orm/prisma-schema/data-model/relations/one-to-many-relations
- Keywords: prisma, prisma schema, orm, docs, docs, orm, prisma schema, data model, relations, one to many relations
- Summary: This page introduces one-to-many relations and explains how to use them in your Prisma schema.
----

Source: https://www.prisma.io/docs/orm/prisma-schema/data-model/relations/one-to-many-relations

This page introduces one-to-many relations and explains how to use them in your Prisma schema.

Questions answered in this page

Overview[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/one-to-many-relations#overview "Direct link to Overview")
--------------------------------------------------------------------------------------------------------------------------------------

One-to-many (1-n) relations refer to relations where one record on one side of the relation can be connected to zero or more records on the other side. In the following example, there is one one-to-many relation between the `User` and `Post` models:

*   Relational databases
*   MongoDB

`model User {  id    Int    @id @default(autoincrement())  posts Post[]}model Post {  id       Int  @id @default(autoincrement())  author   User @relation(fields: [authorId], references: [id])  authorId Int}`

> **Note** The `posts` field does not "manifest" in the underlying database schema. On the other side of the relation, the [annotated relation field](https://www.prisma.io/docs/orm/prisma-schema/data-model/relations#relation-fields)`author` and its relation scalar `authorId` represent the side of the relation that stores the foreign key in the underlying database.

This one-to-many relation expresses the following:

*   "a user can have zero or more posts"
*   "a post must always have an author"

In the previous example, the `author` relation field of the `Post` model references the `id` field of the `User` model. You can also reference a different field. In this case, you need to mark the field with the `@unique` attribute, to guarantee that there is only a single `User` connected to each `Post`. In the following example, the `author` field references an `email` field in the `User` model, which is marked with the `@unique` attribute:

*   Relational databases
*   MongoDB

`model User {  id    Int    @id @default(autoincrement())  email String @unique // <-- add unique attribute  posts Post[]}model Post {  id          Int    @id @default(autoincrement())  authorEmail String  author      User   @relation(fields: [authorEmail], references: [email])}`

warning

In MySQL, you can create a foreign key with only an index on the referenced side, and not a unique constraint. In Prisma ORM versions 4.0.0 and later, if you introspect a relation of this type it will trigger a validation error. To fix this, you will need to add a `@unique` constraint to the referenced field.

Multi-field relations in relational databases[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/one-to-many-relations#multi-field-relations-in-relational-databases "Direct link to Multi-field relations in relational databases")
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

In **relational databases only**, you can also define this relation using [multi-field IDs](https://www.prisma.io/docs/orm/reference/prisma-schema-reference#id-1)/composite key:

`model User {  firstName String  lastName  String  post      Post[]  @@id([firstName, lastName])}model Post {  id              Int    @id @default(autoincrement())  author          User   @relation(fields: [authorFirstName, authorLastName], references: [firstName, lastName])  authorFirstName String // relation scalar field (used in the `@relation` attribute above)  authorLastName  String // relation scalar field (used in the `@relation` attribute above)}`

1-n relations in the database[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/one-to-many-relations#1-n-relations-in-the-database "Direct link to 1-n relations in the database")
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

### Relational databases[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/one-to-many-relations#relational-databases "Direct link to Relational databases")

The following example demonstrates how to create a 1-n relation in SQL:

`CREATE TABLE "User" (    id SERIAL PRIMARY KEY);CREATE TABLE "Post" (    id SERIAL PRIMARY KEY,    "authorId" integer NOT NULL,    FOREIGN KEY ("authorId") REFERENCES "User"(id));`

Since there's no `UNIQUE` constraint on the `authorId` column (the foreign key), you can create **multiple `Post` records that point to the same `User` record**. This makes the relation a one-to-many rather than a one-to-one.

The following example demonstrates how to create a 1-n relation in SQL using a composite key (`firstName` and `lastName`):

`CREATE TABLE "User" (    firstName TEXT,    lastName TEXT,    PRIMARY KEY ("firstName","lastName"));CREATE TABLE "Post" (    id SERIAL PRIMARY KEY,    "authorFirstName" TEXT NOT NULL,    "authorLastName" TEXT NOT NULL,    FOREIGN KEY ("authorFirstName", "authorLastName") REFERENCES "User"("firstName", "lastName"));`

#### Comparing one-to-one and one-to-many relations[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/one-to-many-relations#comparing-one-to-one-and-one-to-many-relations "Direct link to Comparing one-to-one and one-to-many relations")

In relational databases, the main difference between a 1-1 and a 1-n-relation is that in a 1-1-relation the foreign key must have a `UNIQUE` constraint defined on it.

### MongoDB[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/one-to-many-relations#mongodb "Direct link to MongoDB")

For MongoDB, Prisma ORM currently uses a [normalized data model design](https://www.mongodb.com/docs/manual/data-modeling/), which means that documents reference each other by ID in a similar way to relational databases.

The following MongoDB document represents a `User`:

`{ "_id": { "$oid": "60d5922d00581b8f0062e3a8" }, "name": "Ella" }`

Each of the following `Post` MongoDB documents has an `authorId` field which references the same user:

`[  {    "_id": { "$oid": "60d5922e00581b8f0062e3a9" },    "title": "How to make sushi",    "authorId": { "$oid": "60d5922d00581b8f0062e3a8" }  },  {    "_id": { "$oid": "60d5922e00581b8f0062e3aa" },    "title": "How to re-install Windows",    "authorId": { "$oid": "60d5922d00581b8f0062e3a8" }  }]`

#### Comparing one-to-one and one-to-many relations[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/one-to-many-relations#comparing-one-to-one-and-one-to-many-relations-1 "Direct link to Comparing one-to-one and one-to-many relations")

In MongoDB, the only difference between a 1-1 and a 1-n is the number of documents referencing another document in the database - there are no constraints.

Required and optional relation fields in one-to-many relations[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/one-to-many-relations#required-and-optional-relation-fields-in-one-to-many-relations "Direct link to Required and optional relation fields in one-to-many relations")
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

A 1-n-relation always has two relation fields:

*   a [list](https://www.prisma.io/docs/orm/prisma-schema/data-model/models#type-modifiers) relation field which is _not_ annotated with `@relation`
*   the [annotated relation field](https://www.prisma.io/docs/orm/prisma-schema/data-model/relations#annotated-relation-fields) (including its relation scalar)

The annotated relation field and relation scalar of a 1-n relation can either _both_ be optional, or _both_ be mandatory. On the other side of the relation, the list is **always mandatory**.

### Optional one-to-many relation[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/one-to-many-relations#optional-one-to-many-relation "Direct link to Optional one-to-many relation")

In the following example, you can create a `Post` without assigning a `User`:

*   Relational databases
*   MongoDB

`model User {  id    Int    @id @default(autoincrement())  posts Post[]}model Post {  id       Int   @id @default(autoincrement())  author   User? @relation(fields: [authorId], references: [id])  authorId Int?}`

### Mandatory one-to-many relation[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/one-to-many-relations#mandatory-one-to-many-relation "Direct link to Mandatory one-to-many relation")

In the following example, you must assign a `User` when you create a `Post`:

*   Relational databases
*   MongoDB

`model User {  id    Int    @id @default(autoincrement())  posts Post[]}model Post {  id       Int  @id @default(autoincrement())  author   User @relation(fields: [authorId], references: [id])  authorId Int}`

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery, scoped to prisma-schema docs subtree.
- Conversion path: r.jina.ai markdown proxy.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
