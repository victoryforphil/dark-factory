----
## External Docs Snapshot // prisma

- Captured: 2026-02-16T05:57:22.190Z
- Source root: https://www.prisma.io/docs/orm/prisma-schema
- Source page: /docs/orm/prisma-schema/data-model/relations/many-to-many-relations
- Keywords: prisma, prisma schema, orm, docs, docs, orm, prisma schema, data model, relations, many to many relations
- Summary: This guide explains how to define and use many-to-many (m-n) relationships in Prisma, with examples for both relational databases and MongoDB.
----

Source: https://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations

Quick summary

This guide explains how to define and use many-to-many (m-n) relationships in Prisma, with examples for both relational databases and MongoDB.

Many-to-many (m-n) relations refer to relations where zero or more records on one side of the relation can be connected to zero or more records on the other side.

Prisma schema syntax and the implementation in the underlying database differs between [relational databases](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#relational-databases) and [MongoDB](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#mongodb).

Questions answered in this page

Relational databases[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#relational-databases "Direct link to Relational databases")
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------

In relational databases, m-n-relations are typically modelled via [relation tables](https://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#relation-tables). m-n-relations can be either [explicit](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#explicit-many-to-many-relations) or [implicit](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#implicit-many-to-many-relations) in the Prisma schema. We recommend using [implicit](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#implicit-many-to-many-relations) m-n-relations if you do not need to store any additional meta-data in the relation table itself. You can always migrate to an [explicit](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#explicit-many-to-many-relations) m-n-relation later if needed.

### Explicit many-to-many relations[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#explicit-many-to-many-relations "Direct link to Explicit many-to-many relations")

In an explicit m-n relation, the **relation table is represented as a model in the Prisma schema** and can be used in queries. Explicit m-n relations define three models:

*   Two models with m-n relation, such as `Category` and `Post`.
*   One model that represents the [relation table](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#relation-tables), such as `CategoriesOnPosts` (also sometimes called _JOIN_, _link_ or _pivot_ table) in the underlying database. The fields of a relation table model are both annotated relation fields (`post` and `category`) with a corresponding relation scalar field (`postId` and `categoryId`).

The relation table `CategoriesOnPosts` connects related `Post` and `Category` records. In this example, the model representing the relation table also **defines additional fields** that describe the `Post`/`Category` relationship - who assigned the category (`assignedBy`), and when the category was assigned (`assignedAt`):

`model Post {  id         Int                 @id @default(autoincrement())  title      String  categories CategoriesOnPosts[]}model Category {  id    Int                 @id @default(autoincrement())  name  String  posts CategoriesOnPosts[]}model CategoriesOnPosts {  post       Post     @relation(fields: [postId], references: [id])  postId     Int // relation scalar field (used in the `@relation` attribute above)  category   Category @relation(fields: [categoryId], references: [id])  categoryId Int // relation scalar field (used in the `@relation` attribute above)  assignedAt DateTime @default(now())  assignedBy String  @@id([postId, categoryId])}`

The underlying SQL looks like this:

`CREATE TABLE "Post" (    "id" SERIAL NOT NULL,    "title" TEXT NOT NULL,    CONSTRAINT "Post_pkey" PRIMARY KEY ("id"));CREATE TABLE "Category" (    "id" SERIAL NOT NULL,    "name" TEXT NOT NULL,    CONSTRAINT "Category_pkey" PRIMARY KEY ("id"));-- Relation table + indexes --CREATE TABLE "CategoriesOnPosts" (    "postId" INTEGER NOT NULL,    "categoryId" INTEGER NOT NULL,    "assignedAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,    "assignedBy" TEXT NOT NULL,    CONSTRAINT "CategoriesOnPosts_pkey" PRIMARY KEY ("postId","categoryId"));ALTER TABLE "CategoriesOnPosts" ADD CONSTRAINT "CategoriesOnPosts_postId_fkey" FOREIGN KEY ("postId") REFERENCES "Post"("id") ON DELETE RESTRICT ON UPDATE CASCADE;ALTER TABLE "CategoriesOnPosts" ADD CONSTRAINT "CategoriesOnPosts_categoryId_fkey" FOREIGN KEY ("categoryId") REFERENCES "Category"("id") ON DELETE RESTRICT ON UPDATE CASCADE;`

Note that the same rules as for [1-n relations](https://www.prisma.io/docs/orm/prisma-schema/data-model/relations/one-to-many-relations) apply (because `Post`↔ `CategoriesOnPosts` and `Category` ↔ `CategoriesOnPosts` are both in fact 1-n relations), which means one side of the relation needs to be annotated with the `@relation` attribute.

When you don't need to attach additional information to the relation, you can model m-n-relations as [implicit m-n-relations](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#implicit-many-to-many-relations). If you're not using Prisma Migrate but obtain your data model from [introspection](https://www.prisma.io/docs/orm/prisma-schema/introspection), you can still make use of implicit m-n-relations by following Prisma ORM's [conventions for relation tables](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#conventions-for-relation-tables-in-implicit-m-n-relations).

#### Querying an explicit many-to-many[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#querying-an-explicit-many-to-many "Direct link to Querying an explicit many-to-many")

The following section demonstrates how to query an explicit m-n-relation. You can query the relation model directly (`prisma.categoriesOnPosts(...)`), or use nested queries to go from `Post` ->`CategoriesOnPosts` ->`Category` or the other way.

The following query does three things:

1.   Creates a `Post`
2.   Creates a new record in the relation table `CategoriesOnPosts`
3.   Creates a new `Category` that is associated with the newly created `Post` record

`const createCategory = await prisma.post.create({  data: {    title: 'How to be Bob',    categories: {      create: [        {          assignedBy: 'Bob',          assignedAt: new Date(),          category: {            create: {              name: 'New category',            },          },        },      ],    },  },})`

The following query:

*   Creates a new `Post`
*   Creates a new record in the relation table `CategoriesOnPosts`
*   Connects the category assignment to existing categories (with IDs `9` and `22`)

`const assignCategories = await prisma.post.create({  data: {    title: 'How to be Bob',    categories: {      create: [        {          assignedBy: 'Bob',          assignedAt: new Date(),          category: {            connect: {              id: 9,            },          },        },        {          assignedBy: 'Bob',          assignedAt: new Date(),          category: {            connect: {              id: 22,            },          },        },      ],    },  },})`

Sometimes you might not know if a `Category` record exists. If the `Category` record exists, you want to connect a new `Post` record to that category. If the `Category` record does not exist, you want to create the record first and then connect it to the new `Post` record. The following query:

1.   Creates a new `Post`
2.   Creates a new record in the relation table `CategoriesOnPosts`
3.   Connects the category assignment to an existing category (with ID `9`), or creates a new category first if it does not exist

`const assignCategories = await prisma.post.create({  data: {    title: 'How to be Bob',    categories: {      create: [        {          assignedBy: 'Bob',          assignedAt: new Date(),          category: {            connectOrCreate: {              where: {                id: 9,              },              create: {                name: 'New Category',                id: 9,              },            },          },        },      ],    },  },})`

The following query returns all `Post` records where at least one (`some`) category assignment (`categories`) refers to a category named `"New category"`:

`const getPosts = await prisma.post.findMany({  where: {    categories: {      some: {        category: {          name: 'New Category',        },      },    },  },})`

The following query returns all categories where at least one (`some`) related `Post` record titles contain the words `"Cool stuff"`_and_ the category was assigned by Bob.

`const getAssignments = await prisma.category.findMany({  where: {    posts: {      some: {        assignedBy: 'Bob',        post: {          title: {            contains: 'Cool stuff',          },        },      },    },  },})`

The following query gets all category assignments (`CategoriesOnPosts`) records that were assigned by `"Bob"` to one of 5 posts:

`const getAssignments = await prisma.categoriesOnPosts.findMany({  where: {    assignedBy: 'Bob',    post: {      id: {        in: [9, 4, 10, 12, 22],      },    },  },})`

### Implicit many-to-many relations[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#implicit-many-to-many-relations "Direct link to Implicit many-to-many relations")

Implicit m-n relations define relation fields as lists on both sides of the relation. Although the relation table exists in the underlying database, **it is managed by Prisma ORM and does not manifest in the Prisma schema**. Implicit relation tables follow a [specific convention](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#conventions-for-relation-tables-in-implicit-m-n-relations).

Implicit m-n-relations makes the [Prisma Client API](https://www.prisma.io/docs/orm/prisma-client) for m-n-relations a bit simpler (since you have one fewer level of nesting inside of [nested writes](https://www.prisma.io/docs/orm/prisma-client/queries/relation-queries#nested-writes)).

In the example below, there's one _implicit_ m-n-relation between `Post` and `Category`:

*   Relational databases
*   MongoDB

`model Post {  id         Int        @id @default(autoincrement())  title      String  categories Category[]}model Category {  id    Int    @id @default(autoincrement())  name  String  posts Post[]}`

#### Querying an implicit many-to-many[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#querying-an-implicit-many-to-many "Direct link to Querying an implicit many-to-many")

The following section demonstrates how to query an [implicit m-n](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#implicit-many-to-many-relations) relation. The queries require less nesting than [explicit m-n queries](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#querying-an-explicit-many-to-many).

The following query creates a single `Post` and multiple `Category` records:

`const createPostAndCategory = await prisma.post.create({  data: {    title: 'How to become a butterfly',    categories: {      create: [{ name: 'Magic' }, { name: 'Butterflies' }],    },  },})`

The following query creates a single `Category` and multiple `Post` records:

`const createCategoryAndPosts = await prisma.category.create({  data: {    name: 'Stories',    posts: {      create: [        { title: 'That one time with the stuff' },        { title: 'The story of planet Earth' },      ],    },  },})`

The following query returns all `Post` records with a list of that post's assigned categories:

`const getPostsAndCategories = await prisma.post.findMany({  include: {    categories: true,  },})`

#### Rules for defining an implicit m-n relation[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#rules-for-defining-an-implicit-m-n-relation "Direct link to Rules for defining an implicit m-n relation")

Implicit m-n relations:

*   Use a specific [convention for relation tables](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#conventions-for-relation-tables-in-implicit-m-n-relations)

*   Do **not** require the `@relation` attribute unless you need to [disambiguate relations](https://www.prisma.io/docs/orm/prisma-schema/data-model/relations#disambiguating-relations) with a name, e.g. `@relation("MyRelation")` or `@relation(name: "MyRelation")`.

*   If you do use the `@relation` attribute, you cannot use the `references`, `fields`, `onUpdate` or `onDelete` arguments. This is because these take a fixed value for implicit m-n-relations and cannot be changed.

*   Require both models to have a single `@id`. Be aware that:

    *   You cannot use a [multi-field ID](https://www.prisma.io/docs/orm/reference/prisma-schema-reference#id-1)
    *   You cannot use a `@unique` in place of an `@id`

info

To use either of these features, you must use an [explicit m-n instead](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#explicit-many-to-many-relations).

#### Conventions for relation tables in implicit m-n relations[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#conventions-for-relation-tables-in-implicit-m-n-relations "Direct link to Conventions for relation tables in implicit m-n relations")

If you obtain your data model from [introspection](https://www.prisma.io/docs/orm/prisma-schema/introspection), you can still use implicit m-n-relations by following Prisma ORM's [conventions for relation tables](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#conventions-for-relation-tables-in-implicit-m-n-relations). The following example assumes you want to create a relation table to get an implicit m-n-relation for two models called `Post` and `Category`.

##### Relation table[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#relation-table "Direct link to Relation table")

If you want a relation table to be picked up by introspection as an implicit m-n-relation, the name must follow this exact structure:

*   It must start with an underscore `_`
*   Then the name of the first model in alphabetical order (in this case `Category`)
*   Then the relationship (in this case `To`)
*   Then the name of the second model in alphabetical order (in this case `Post`)

In the example, the correct table name is `_CategoryToPost`.

When creating an implicit m-n-relation yourself in the Prisma schema file, you can [configure the relation](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#configuring-the-name-of-the-relation-table-in-implicit-many-to-many-relations) to have a different name. This will change the name given to the relation table in the database. For example, for a relation named `"MyRelation"` the corresponding table will be called `_MyRelation`.

###### Multi-schema[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#multi-schema "Direct link to Multi-schema")

If your implicit many-to-many relationship spans multiple database schemas (using the [`multiSchema` feature](https://www.prisma.io/docs/orm/prisma-schema/data-model/multi-schema)), the relation table (with the name defined directly above, in the example `_CategoryToPost`) must be present in the same database schema as the first model in alphabetical order (in this case `Category`).

##### Columns[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#columns "Direct link to Columns")

A relation table for an implicit m-n-relation must have exactly two columns:

*   A foreign key column that points to `Category` called `A`
*   A foreign key column that points to `Post` called `B`

The columns must be called `A` and `B` where `A` points to the model that comes first in the alphabet and `B` points to the model which comes last in the alphabet.

##### Indexes[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#indexes "Direct link to Indexes")

There further must be:

*   A unique index defined on both foreign key columns:

`CREATE UNIQUE INDEX "_CategoryToPost_AB_unique" ON "_CategoryToPost"("A" int4_ops,"B" int4_ops);` 
*   A non-unique index defined on B:

`CREATE INDEX "_CategoryToPost_B_index" ON "_CategoryToPost"("B" int4_ops);` 

##### Example[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#example "Direct link to Example")

This is a sample SQL statement that would create the three tables including indexes (in PostgreSQL dialect) that are picked up as a implicit m-n-relation by Prisma Introspection:

`CREATE TABLE "_CategoryToPost" (    "A" integer NOT NULL REFERENCES "Category"(id) ,    "B" integer NOT NULL REFERENCES "Post"(id));CREATE UNIQUE INDEX "_CategoryToPost_AB_unique" ON "_CategoryToPost"("A" int4_ops,"B" int4_ops);CREATE INDEX "_CategoryToPost_B_index" ON "_CategoryToPost"("B" int4_ops);CREATE TABLE "Category" (    id integer SERIAL PRIMARY KEY);CREATE TABLE "Post" (    id integer SERIAL PRIMARY KEY);`

And you can define multiple many-to-many relations between two tables by using the different relationship name. This example shows how the Prisma introspection works under such case:

`CREATE TABLE IF NOT EXISTS "User" (    "id" SERIAL PRIMARY KEY);CREATE TABLE IF NOT EXISTS "Video" (    "id" SERIAL PRIMARY KEY);CREATE TABLE IF NOT EXISTS "_UserLikedVideos" (    "A" SERIAL NOT NULL,    "B" SERIAL NOT NULL,    CONSTRAINT "_UserLikedVideos_A_fkey" FOREIGN KEY ("A") REFERENCES "User" ("id") ON DELETE CASCADE ON UPDATE CASCADE,    CONSTRAINT "_UserLikedVideos_B_fkey" FOREIGN KEY ("B") REFERENCES "Video" ("id") ON DELETE CASCADE ON UPDATE CASCADE);CREATE TABLE IF NOT EXISTS "_UserDislikedVideos" (    "A" SERIAL NOT NULL,    "B" SERIAL NOT NULL,    CONSTRAINT "_UserDislikedVideos_A_fkey" FOREIGN KEY ("A") REFERENCES "User" ("id") ON DELETE CASCADE ON UPDATE CASCADE,    CONSTRAINT "_UserDislikedVideos_B_fkey" FOREIGN KEY ("B") REFERENCES "Video" ("id") ON DELETE CASCADE ON UPDATE CASCADE);CREATE UNIQUE INDEX "_UserLikedVideos_AB_unique" ON "_UserLikedVideos"("A", "B");CREATE INDEX "_UserLikedVideos_B_index" ON "_UserLikedVideos"("B");CREATE UNIQUE INDEX "_UserDislikedVideos_AB_unique" ON "_UserDislikedVideos"("A", "B");CREATE INDEX "_UserDislikedVideos_B_index" ON "_UserDislikedVideos"("B");`

If you run `prisma db pull` on this database, the Prisma CLI will generate the following schema through introspection:

`model User {  id                       Int     @id @default(autoincrement())  Video_UserDislikedVideos Video[] @relation("UserDislikedVideos")  Video_UserLikedVideos    Video[] @relation("UserLikedVideos")}model Video {  id                      Int    @id @default(autoincrement())  User_UserDislikedVideos User[] @relation("UserDislikedVideos")  User_UserLikedVideos    User[] @relation("UserLikedVideos")}`

#### Configuring the name of the relation table in implicit many-to-many relations[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#configuring-the-name-of-the-relation-table-in-implicit-many-to-many-relations "Direct link to Configuring the name of the relation table in implicit many-to-many relations")

When using Prisma Migrate, you can configure the name of the relation table that's managed by Prisma ORM using the `@relation` attribute. For example, if you want the relation table to be called `_MyRelationTable` instead of the default name `_CategoryToPost`, you can specify it as follows:

`model Post {  id         Int        @id @default(autoincrement())  categories Category[] @relation("MyRelationTable")}model Category {  id    Int    @id @default(autoincrement())  posts Post[] @relation("MyRelationTable")}`

### Relation tables[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#relation-tables "Direct link to Relation tables")

A relation table (also sometimes called a _JOIN_, _link_ or _pivot_ table) connects two or more other tables and therefore creates a _relation_ between them. Creating relation tables is a common data modelling practice in SQL to represent relationships between different entities. In essence it means that "one m-n relation is modeled as two 1-n relations in the database".

We recommend using [implicit](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#implicit-many-to-many-relations) m-n-relations, where Prisma ORM automatically generates the relation table in the underlying database. [Explicit](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#explicit-many-to-many-relations) m-n-relations should be used when you need to store additional data in the relations, such as the date the relation was created.

MongoDB[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#mongodb "Direct link to MongoDB")
------------------------------------------------------------------------------------------------------------------------------------

In MongoDB, m-n-relations are represented by:

*   relation fields on both sides, that each have a `@relation` attribute, with mandatory `fields` and `references` arguments
*   a scalar list of referenced IDs on each side, with a type that matches the ID field on the other side

The following example demonstrates a m-n-relation between posts and categories:

`model Post {  id          String     @id @default(auto()) @map("_id") @db.ObjectId  categoryIDs String[]   @db.ObjectId  categories  Category[] @relation(fields: [categoryIDs], references: [id])}model Category {  id      String   @id @default(auto()) @map("_id") @db.ObjectId  name    String  postIDs String[] @db.ObjectId  posts   Post[]   @relation(fields: [postIDs], references: [id])}`

Prisma ORM validates m-n-relations in MongoDB with the following rules:

*   The fields on both sides of the relation must have a list type (in the example above, `categories` have a type of `Category[]` and `posts` have a type of `Post[]`)
*   The `@relation` attribute must define `fields` and `references` arguments on both sides
*   The `fields` argument must have only one scalar field defined, which must be of a list type
*   The `references` argument must have only one scalar field defined. This scalar field must exist on the referenced model and must be of the same type as the scalar field in the `fields` argument, but singular (no list)
*   The scalar field to which `references` points must have the `@id` attribute
*   No [referential actions](https://www.prisma.io/docs/orm/prisma-schema/data-model/relations/referential-actions) are allowed in `@relation`

The implicit m-n-relations [used in relational databases](https://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#implicit-many-to-many-relations) are not supported on MongoDB.

### Querying MongoDB many-to-many relations[​](http://www.prisma.io/docs/orm/prisma-schema/data-model/relations/many-to-many-relations#querying-mongodb-many-to-many-relations "Direct link to Querying MongoDB many-to-many relations")

This section demonstrates how to query m-n-relations in MongoDB, using the example schema above.

The following query finds posts with specific matching category IDs:

`const newId1 = new ObjectId()const newId2 = new ObjectId()const posts = await prisma.post.findMany({  where: {    categoryIDs: {      hasSome: [newId1.toHexString(), newId2.toHexString()],    },  },})`

The following query finds posts where the category name contains the string `'Servers'`:

`const posts = await prisma.post.findMany({  where: {    categories: {      some: {        name: {          contains: 'Servers',        },      },    },  },})`

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery, scoped to prisma-schema docs subtree.
- Conversion path: r.jina.ai markdown proxy.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
