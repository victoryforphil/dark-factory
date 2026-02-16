----
## External Docs Snapshot // prisma

- Captured: 2026-02-16T05:57:22.190Z
- Source root: https://www.prisma.io/docs/orm/prisma-schema
- Source page: /docs/orm/prisma-schema/data-model/relations/referential-actions
- Keywords: prisma, prisma schema, orm, docs, docs, orm, prisma schema, data model, relations, referential actions
- Summary: Referential actions determine what happens to a record when your application deletes or updates a related record.
----

Source: https://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions

Referential actions determine what happens to a record when your application deletes or updates a related record.

From version 2.26.0, you can define referential actions on the relation fields in your Prisma schema. This allows you to define referential actions like cascading deletes and cascading updates at a Prisma ORM level.

info

**Version differences**

*   If you use version 3.0.1 or later, you can use referential actions as described on this page.
*   If you use a version between 2.26.0 and 3.0.0, you can use referential actions as described on this page, but you must [enable the preview feature flag](https://www.prisma.io/docs/orm/reference/preview-features/client-preview-features#enabling-a-prisma-client-preview-feature)`referentialActions`.
*   If you use version 2.25.0 or earlier, you can configure cascading deletes manually in your database.

In the following example, adding `onDelete: Cascade` to the `author` field on the `Post` model means that deleting the `User` record will also delete all related `Post` records.

schema.prisma

`model Post {  id       Int    @id @default(autoincrement())  title    String  author   User   @relation(fields: [authorId], references: [id], onDelete: Cascade)  authorId Int}model User {  id    Int    @id @default(autoincrement())  posts Post[]}`

If you do not specify a referential action, Prisma ORM [uses a default](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#referential-action-defaults).

Questions answered in this page

danger

If you upgrade from a version earlier than 2.26.0: It is extremely important that you check the [upgrade paths for referential actions](https://www.prisma.io/docs/orm/more/upgrade-guides/upgrading-versions/upgrading-to-prisma-3/referential-actions) section. Prisma ORM's support of referential actions **removes the safety net in Prisma Client that prevents cascading deletes at runtime**. If you use the feature _without upgrading your database_, the [old default action](https://www.prisma.io/docs/orm/more/upgrade-guides/upgrading-versions/upgrading-to-prisma-3/referential-actions#prisma-orm-2x-default-referential-actions) - `ON DELETE CASCADE` - becomes active. This might result in cascading deletes that you did not expect.

What are referential actions?[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#what-are-referential-actions "Direct link to What are referential actions?")
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Referential actions are policies that define how a referenced record is handled by the database when you run an [`update`](https://www.prisma.io/docs/orm/prisma-client/queries/crud#update) or [`delete`](https://www.prisma.io/docs/orm/prisma-client/queries/crud#delete) query.

Referential actions on the database level

### How to use referential actions[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#how-to-use-referential-actions "Direct link to How to use referential actions")

Referential actions are defined in the [`@relation`](https://www.prisma.io/docs/orm/reference/prisma-schema-reference#relation) attribute and map to the actions on the **foreign key constraint** in the underlying database. If you do not specify a referential action, [Prisma ORM falls back to a default](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#referential-action-defaults).

The following model defines a one-to-many relation between `User` and `Post` and a many-to-many relation between `Post` and `Tag`, with explicitly defined referential actions:

schema.prisma

`model User {  id    Int    @id @default(autoincrement())  posts Post[]}model Post {  id     Int          @id @default(autoincrement())  title  String  tags   TagOnPosts[]  User   User?        @relation(fields: [userId], references: [id], onDelete: SetNull, onUpdate: Cascade)  userId Int?}model TagOnPosts {  id     Int   @id @default(autoincrement())  post   Post? @relation(fields: [postId], references: [id], onUpdate: Cascade, onDelete: Cascade)  tag    Tag?  @relation(fields: [tagId], references: [id], onUpdate: Cascade, onDelete: Cascade)  postId Int?  tagId  Int?}model Tag {  id    Int          @id @default(autoincrement())  name  String       @unique  posts TagOnPosts[]}`

This model explicitly defines the following referential actions:

*   If you delete a `Tag`, the corresponding tag assignment is also deleted in `TagOnPosts`, using the `Cascade` referential action
*   If you delete a `User`, the author is removed from all posts by setting the field value to `Null`, because of the `SetNull` referential action. To allow this, `User` and `userId` must be optional fields in `Post`.

Prisma ORM supports the following referential actions:

*   [`Cascade`](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#cascade)
*   [`Restrict`](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#restrict)
*   [`NoAction`](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#noaction)
*   [`SetNull`](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#setnull)
*   [`SetDefault`](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#setdefault)

### Referential action defaults[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#referential-action-defaults "Direct link to Referential action defaults")

If you do not specify a referential action, Prisma ORM uses the following defaults:

| Clause | Optional relations | Mandatory relations |
| --- | --- | --- |
| `onDelete` | `SetNull` | `Restrict` |
| `onUpdate` | `Cascade` | `Cascade` |

For example, in the following schema all `Post` records must be connected to a `User` via the `author` relation:

`model Post {  id       Int    @id @default(autoincrement())  title    String  author   User   @relation(fields: [authorId], references: [id])  authorId Int}model User {  id    Int    @id @default(autoincrement())  posts Post[]}`

The schema does not explicitly define referential actions on the mandatory `author` relation field, which means that the default referential actions of `Restrict` for `onDelete` and `Cascade` for `onUpdate` apply.

Caveats[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#caveats "Direct link to Caveats")
---------------------------------------------------------------------------------------------------------------------------------

The following caveats apply:

*   Referential actions are **not** supported on [implicit many-to-many relations](https://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#implicit-many-to-many-relations). To use referential actions, you must define an explicit many-to-many relation and define your referential actions on the [join table](https://www.prisma.io/docs/orm/prisma-schema/data-model/relations/troubleshooting-relations#how-to-use-a-relation-table-with-a-many-to-many-relationship).
*   Certain combinations of referential actions and required/optional relations are incompatible. For example, using `SetNull` on a required relation will lead to database errors when deleting referenced records because the non-nullable constraint would be violated. See [this GitHub issue](https://github.com/prisma/prisma/issues/7909) for more information.

Types of referential actions[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#types-of-referential-actions "Direct link to Types of referential actions")
------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

The following table shows which referential action each database supports.

| Database | Cascade | Restrict | NoAction | SetNull | SetDefault |
| --- | --- | --- | --- | --- | --- |
| PostgreSQL | ✔️ | ✔️ | ✔️ | ✔️⌘ | ✔️ |
| MySQL/MariaDB | ✔️ | ✔️ | ✔️ | ✔️ | ❌ (✔️†) |
| SQLite | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ |
| SQL Server | ✔️ | ❌‡ | ✔️ | ✔️ | ✔️ |
| CockroachDB | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ |
| MongoDB†† | ✔️ | ✔️ | ✔️ | ✔️ | ❌ |

*   † See [special cases for MySQL](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#mysqlmariadb).
*   ⌘ See [special cases for PostgreSQL](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#postgresql).
*   ‡ See [special cases for SQL Server](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#sql-server).
*   †† Referential actions for MongoDB are available in Prisma ORM versions 3.7.0 and later.

### Special cases for referential actions[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#special-cases-for-referential-actions "Direct link to Special cases for referential actions")

Referential actions are part of the ANSI SQL standard. However, there are special cases where some relational databases diverge from the standard.

#### MySQL/MariaDB[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#mysqlmariadb "Direct link to MySQL/MariaDB")

MySQL/MariaDB, and the underlying InnoDB storage engine, does not support `SetDefault`. The exact behavior depends on the database version:

*   In MySQL versions 8 and later, and MariaDB versions 10.5 and later, `SetDefault` effectively acts as an alias for `NoAction`. You can define tables using the `SET DEFAULT` referential action, but a foreign key constraint error is triggered at runtime.
*   In MySQL versions 5.6 and later, and MariaDB versions before 10.5, attempting to create a table definition with the `SET DEFAULT` referential action fails with a syntax error.

For this reason, when you set `mysql` as the database provider, Prisma ORM warns users to replace `SetDefault` referential actions in the Prisma schema with another action.

#### PostgreSQL[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#postgresql "Direct link to PostgreSQL")

PostgreSQL is the only database supported by Prisma ORM that allows you to define a `SetNull` referential action that refers to a non-nullable field. However, this raises a foreign key constraint error when the action is triggered at runtime.

For this reason, when you set `postgres` as the database provider in the (default) `foreignKeys` relation mode, Prisma ORM warns users to mark as optional any fields that are included in a `@relation` attribute with a `SetNull` referential action. For all other database providers, Prisma ORM rejects the schema with a validation error.

#### SQL Server[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#sql-server "Direct link to SQL Server")

[`Restrict`](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#restrict) is not available for SQL Server databases, but you can use [`NoAction`](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#noaction) instead.

### `Cascade`[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#cascade "Direct link to cascade")

*   `onDelete: Cascade` Deleting a referenced record will trigger the deletion of referencing record.
*   `onUpdate: Cascade` Updates the relation scalar fields if the referenced scalar fields of the dependent record are updated.

#### Example usage[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#example-usage "Direct link to Example usage")

schema.prisma

`model Post {  id       Int    @id @default(autoincrement())  title    String  author   User   @relation(fields: [authorId], references: [id], onDelete: Cascade, onUpdate: Cascade)  authorId Int}model User {  id    Int    @id @default(autoincrement())  posts Post[]}`

##### Result of using `Cascade`[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#result-of-using-cascade "Direct link to result-of-using-cascade")

If a `User` record is deleted, then their posts are deleted too. If the user's `id` is updated, then the corresponding `authorId` is also updated.

##### How to use cascading deletes[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#how-to-use-cascading-deletes "Direct link to How to use cascading deletes")

### `Restrict`[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#restrict "Direct link to restrict")

*   `onDelete: Restrict` Prevents the deletion if any referencing records exist.
*   `onUpdate: Restrict` Prevents the identifier of a referenced record from being changed.

#### Example usage[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#example-usage-1 "Direct link to Example usage")

schema.prisma

`model Post {  id       Int    @id @default(autoincrement())  title    String  author   User   @relation(fields: [authorId], references: [id], onDelete: Restrict, onUpdate: Restrict)  authorId Int}model User {  id    Int    @id @default(autoincrement())  posts Post[]}`

##### Result of using `Restrict`[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#result-of-using-restrict "Direct link to result-of-using-restrict")

`User`s with posts **cannot** be deleted. The `User`'s `id`**cannot** be changed.

warning

The `Restrict` action is **not** available on [Microsoft SQL Server](https://www.prisma.io/docs/orm/overview/databases/sql-server) and triggers a schema validation error. Instead, you can use [`NoAction`](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#noaction), which produces the same result and is compatible with SQL Server.

### `NoAction`[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#noaction "Direct link to noaction")

The `NoAction` action is similar to `Restrict`, the difference between the two is dependent on the database being used:

*   **PostgreSQL**: `NoAction` allows the check (if a referenced row on the table exists) to be deferred until later in the transaction. See [the PostgreSQL docs](https://www.postgresql.org/docs/current/ddl-constraints.html#DDL-CONSTRAINTS-FK) for more information.
*   **MySQL**: `NoAction` behaves exactly the same as `Restrict`. See [the MySQL docs](https://dev.mysql.com/doc/refman/8.0/en/create-table-foreign-keys.html#foreign-key-referential-actions) for more information.
*   **SQLite**: When a related primary key is modified or deleted, no action is taken. See [the SQLite docs](https://www.sqlite.org/foreignkeys.html#fk_actions) for more information.
*   **SQL Server**: When a referenced record is deleted or modified, an error is raised. See [the SQL Server docs](https://learn.microsoft.com/en-us/sql/relational-databases/tables/graph-edge-constraints?view=sql-server-ver15#on-delete-referential-actions-on-edge-constraints) for more information.
*   **MongoDB** (in preview from version 3.6.0): When a record is modified or deleted, nothing is done to any related records.

warning

If you are [managing relations in Prisma Client](https://www.prisma.io/docs/orm/prisma-schema/data-model/relations/relation-mode#emulate-relations-in-prisma-orm-with-the-prisma-relation-mode) rather than using foreign keys in the database, you should be aware that currently Prisma ORM only implements the referential actions. Foreign keys also create constraints, which make it impossible to manipulate data in a way that would violate these constraints: instead of executing the query, the database responds with an error. These constraints will not be created if you emulate referential integrity in Prisma Client, so if you set the referential action to `NoAction` there will be no checks to prevent you from breaking the referential integrity.

#### Example usage[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#example-usage-2 "Direct link to Example usage")

schema.prisma

`model Post {  id       Int    @id @default(autoincrement())  title    String  author   User   @relation(fields: [authorId], references: [id], onDelete: NoAction, onUpdate: NoAction)  authorId Int}model User {  id    Int    @id @default(autoincrement())  posts Post[]}`

##### Result of using `NoAction`[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#result-of-using-noaction "Direct link to result-of-using-noaction")

`User`'s with posts **cannot** be deleted. The `User`'s `id`**cannot** be changed.

### `SetNull`[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#setnull "Direct link to setnull")

*   `onDelete: SetNull` The scalar field of the referencing object will be set to `NULL`.

*   `onUpdate: SetNull` When updating the identifier of a referenced object, the scalar fields of the referencing objects will be set to `NULL`.

`SetNull` will only work on optional relations. On required relations, a runtime error will be thrown since the scalar fields cannot be null.

schema.prisma

`model Post {  id       Int    @id @default(autoincrement())  title    String  author   User?  @relation(fields: [authorId], references: [id], onDelete: SetNull, onUpdate: SetNull)  authorId Int?}model User {  id    Int    @id @default(autoincrement())  posts Post[]}`

##### Result of using `SetNull`[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#result-of-using-setnull "Direct link to result-of-using-setnull")

When deleting a `User`, the `authorId` will be set to `NULL` for all its authored posts.

When changing a `User`'s `id`, the `authorId` will be set to `NULL` for all its authored posts.

### `SetDefault`[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#setdefault "Direct link to setdefault")

*   `onDelete: SetDefault` The scalar field of the referencing object will be set to the fields default value.

*   `onUpdate: SetDefault` The scalar field of the referencing object will be set to the fields default value.

These require setting a default for the relation scalar field with [`@default`](https://www.prisma.io/docs/orm/reference/prisma-schema-reference#default). If no defaults are provided for any of the scalar fields, a runtime error will be thrown.

schema.prisma

`model Post {  id             Int     @id @default(autoincrement())  title          String  authorUsername String? @default("anonymous")  author         User?   @relation(fields: [authorUsername], references: [username], onDelete: SetDefault, onUpdate: SetDefault)}model User {  username String @id  posts    Post[]}`

##### Result of using `SetDefault`[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#result-of-using-setdefault "Direct link to result-of-using-setdefault")

When deleting a `User`, its existing posts' `authorUsername` field values will be set to 'anonymous'.

When the `username` of a `User` changes, its existing posts' `authorUsername` field values will be set to 'anonymous'.

### Database-specific requirements[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#database-specific-requirements "Direct link to Database-specific requirements")

MongoDB and SQL Server have specific requirements for referential actions if you have [self-relations](https://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions/special-rules-for-referential-actions#self-relation-sql-server-and-mongodb) or [cyclic relations](https://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions/special-rules-for-referential-actions#cyclic-relation-between-three-tables-sql-server-and-mongodb) in your data model. SQL Server also has specific requirements if you have relations with [multiple cascade paths](https://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions/special-rules-for-referential-actions#multiple-cascade-paths-between-two-models-sql-server-only).

Upgrade paths from versions 2.25.0 and earlier[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#upgrade-paths-from-versions-2250-and-earlier "Direct link to Upgrade paths from versions 2.25.0 and earlier")
----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

There are a couple of paths you can take when upgrading which will give different results depending on the desired outcome.

If you currently use the migration workflow, you can run an introspection to check how the defaults are reflected in your schema. You can then manually update your database if you need to.

You can also decide to skip checking the defaults and run a migration to update your database with the [new default values](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#referential-action-defaults).

The following assumes you have upgraded to 2.26.0 or newer and enabled the preview feature flag, or upgraded to 3.0.0 or newer:

### Using Introspection[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#using-introspection "Direct link to Using Introspection")

If you [Introspect](https://www.prisma.io/docs/orm/prisma-schema/introspection) your database, the referential actions configured at the database level will be reflected in your Prisma Schema. If you have been using Prisma Migrate or `prisma db push` to manage the database schema, these are likely to be the [default values](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#referential-action-defaults) from 2.25.0 and earlier.

When you run an Introspection, Prisma ORM compares all the foreign keys in the database with the schema, if the SQL statements `ON DELETE` and `ON UPDATE` do **not** match the default values, they will be explicitly set in the schema file.

After introspecting, you can review the non-default clauses in your schema. The most important clause to review is `onDelete`, which defaults to `Cascade` in 2.25.0 and earlier.

warning

If you are using either the [`delete()`](https://www.prisma.io/docs/orm/prisma-client/queries/crud#delete-a-single-record) or [`deleteMany()`](https://www.prisma.io/docs/orm/prisma-client/queries/crud#delete-all-records) methods, **[cascading deletes](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#how-to-use-cascading-deletes) will now be performed** as the `referentialActions` preview feature **removed the safety net in Prisma Client that previously prevented cascading deletes at runtime**. Be sure to check your code and make any adjustments accordingly.

Make sure you are happy with every case of `onDelete: Cascade` in your schema. If not, either:

*   Modify your Prisma schema and `db push` or `dev migrate` to change the database

_or_

*   Manually update the underlying database if you use an introspection-only workflow

The following example would result in a cascading delete, if the `User` is deleted then all of their `Post`'s will be deleted too.

#### A blog schema example[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#a-blog-schema-example "Direct link to A blog schema example")

`model Post {  id       Int    @id @default(autoincrement())  title    String  author   User   @relation(fields: [authorId], references: [id], onDelete: Cascade)  authorId Int}model User {  id    Int    @id @default(autoincrement())  posts Post[]}`

### Using Migration[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#using-migration "Direct link to Using Migration")

When running a [Migration](https://www.prisma.io/docs/orm/prisma-migrate) (or the [`prisma db push`](https://www.prisma.io/docs/orm/prisma-migrate/workflows/prototyping-your-schema) command) the [new defaults](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#referential-action-defaults) will be applied to your database.

info

Unlike when you run an Introspect for the first time, the new referential actions clause and property, will **not** automatically be added to your prisma schema by the Prisma VSCode extension. You will have to manually add them if you wish to use anything other than the new defaults.

Explicitly defining referential actions in your Prisma schema is optional. If you do not explicitly define a referential action for a relation, Prisma ORM uses the [new defaults](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#referential-action-defaults).

Note that referential actions can be added on a case by case basis. This means that you can add them to one single relation and leave the rest set to the defaults by not manually specifying anything.

### Checking for errors[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#checking-for-errors "Direct link to Checking for errors")

**Before** upgrading to 2.26.0 and enabling the referential actions **preview feature**, Prisma ORM prevented the deletion of records while using `delete()` or `deleteMany()` to preserve referential integrity. A custom runtime error would be thrown by Prisma Client with the error code `P2014`.

**After** upgrading and enabling the referential actions **preview feature**, Prisma ORM no longer performs runtime checks. You can instead specify a custom referential action to preserve the referential integrity between relations.

When you use [`NoAction`](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#noaction) or [`Restrict`](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#restrict) to prevent the deletion of records, the error messages will be different post 2.26.0 compared to pre 2.26.0. This is because they are now triggered by the database and **not** Prisma Client. The new error code that can be expected is `P2003`.

To make sure you catch these new errors you can adjust your code accordingly.

#### Example of catching errors[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#example-of-catching-errors "Direct link to Example of catching errors")

The following example uses the below blog schema with a one-to-many relationship between `Post` and `User` and sets a [`Restrict`](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions#restrict) referential actions on the `author` field.

This means that if a user has a post, that user (and their posts) **cannot** be deleted.

schema.prisma

`model Post {  id       Int    @id @default(autoincrement())  title    String  author   User   @relation(fields: [authorId], references: [id], onDelete: Restrict)  authorId String}model User {  id    Int    @id @default(autoincrement())  posts Post[]}`

Prior to upgrading and enabling the referential actions **preview feature**, the error code you would receive when trying to delete a user which has posts would be `P2014` and it's message:

> "The change you are trying to make would violate the required relation '{relation_name}' between the {model_a_name} and {model_b_name} models."

`import { PrismaClient } from '../prisma/generated/client'const prisma = new PrismaClient()async function main() {  try {    await prisma.user.delete({      where: {        id: 'some-long-id',      },    })  } catch (error) {    if (error instanceof Prisma.PrismaClientKnownRequestError) {      if (error.code === 'P2014') {        console.log(error.message)      }    }  }}main()`

To make sure you are checking for the correct errors in your code, modify your check to look for `P2003`, which will deliver the message:

> "Foreign key constraint failed on the field: {field_name}"

`import { PrismaClient } from '../prisma/generated/client'const prisma = new PrismaClient()async function main() {  try {    await prisma.user.delete({      where: {        id: 'some-long-id'      }    })  } catch (error) {    if (error instanceof Prisma.PrismaClientKnownRequestError) {      if (error.code === 'P2014') {      if (error.code === 'P2003') {        console.log(error.message)      }    }  }}main()`

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery, scoped to prisma-schema docs subtree.
- Conversion path: r.jina.ai markdown proxy.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
