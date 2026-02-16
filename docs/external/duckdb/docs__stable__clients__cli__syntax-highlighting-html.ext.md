----
## External Docs Snapshot // duckdb

- Captured: 2026-02-16T04:20:47.070Z
- Source root: https://duckdb.org/docs/stable
- Source page: /docs/stable/clients/cli/syntax_highlighting.html
- Keywords: duckdb, docs, client apis, cli, node neo, rust, wasm, clients, cli, syntax highlighting html
- Summary: Syntax Highlighting – DuckDB
----

Source: https://duckdb.org/docs/stable/clients/cli/syntax_highlighting.html

Syntax Highlighting – DuckDB
===============

[![Image 2: DuckDB Logo for Download](http://duckdb.org/images/logo-dl/DuckDB_Logo-horizontal.svg)](https://duckdb.org/)

⌘+k ctrl+k

1.4 (stable)

*   [1.5-dev"](http://duckdb.org/docs/preview/clients/cli/syntax_highlighting)
*   [1.4](http://duckdb.org/docs/stable/clients/cli/syntax_highlighting)
*   [1.3](http://duckdb.org/docs/1.3/clients/cli/syntax_highlighting)
*   [1.2](http://duckdb.org/docs/1.2/clients/cli/syntax_highlighting)

 Search Shortcut cmd + k | ctrl + k

*   [Installation](http://duckdb.org/docs/stable/installation/index)
*    Documentation 

    *   [Getting Started](http://duckdb.org/docs/stable/index)
    *    Connect 

    
        *   [Overview](http://duckdb.org/docs/stable/connect/overview)
        *   [Concurrency](http://duckdb.org/docs/stable/connect/concurrency)

    *    Data Import and Export 

    
        *   [Overview](http://duckdb.org/docs/stable/data/overview)
        *   [Data Sources](http://duckdb.org/docs/stable/data/data_sources)
        *    CSV Files 

        
            *   [Overview](http://duckdb.org/docs/stable/data/csv/overview)
            *   [Auto Detection](http://duckdb.org/docs/stable/data/csv/auto_detection)
            *   [Reading Faulty CSV Files](http://duckdb.org/docs/stable/data/csv/reading_faulty_csv_files)
            *   [Tips](http://duckdb.org/docs/stable/data/csv/tips)

        *    JSON Files 

        
            *   [Overview](http://duckdb.org/docs/stable/data/json/overview)
            *   [Creating JSON](http://duckdb.org/docs/stable/data/json/creating_json)
            *   [Loading JSON](http://duckdb.org/docs/stable/data/json/loading_json)
            *   [Writing JSON](http://duckdb.org/docs/stable/data/json/writing_json)
            *   [JSON Type](http://duckdb.org/docs/stable/data/json/json_type)
            *   [JSON Functions](http://duckdb.org/docs/stable/data/json/json_functions)
            *   [Format Settings](http://duckdb.org/docs/stable/data/json/format_settings)
            *   [Installing and Loading](http://duckdb.org/docs/stable/data/json/installing_and_loading)
            *   [SQL to / from JSON](http://duckdb.org/docs/stable/data/json/sql_to_and_from_json)
            *   [Caveats](http://duckdb.org/docs/stable/data/json/caveats)

        *    Multiple Files 

        
            *   [Overview](http://duckdb.org/docs/stable/data/multiple_files/overview)
            *   [Combining Schemas](http://duckdb.org/docs/stable/data/multiple_files/combining_schemas)

        *    Parquet Files 

        
            *   [Overview](http://duckdb.org/docs/stable/data/parquet/overview)
            *   [Metadata](http://duckdb.org/docs/stable/data/parquet/metadata)
            *   [Encryption](http://duckdb.org/docs/stable/data/parquet/encryption)
            *   [Tips](http://duckdb.org/docs/stable/data/parquet/tips)

        *    Partitioning 

        
            *   [Hive Partitioning](http://duckdb.org/docs/stable/data/partitioning/hive_partitioning)
            *   [Partitioned Writes](http://duckdb.org/docs/stable/data/partitioning/partitioned_writes)

        *   [Appender](http://duckdb.org/docs/stable/data/appender)
        *   [INSERT Statements](http://duckdb.org/docs/stable/data/insert)

    *   [Lakehouse Formats](http://duckdb.org/docs/stable/lakehouse_formats)
    *    Client APIs 

    
        *   [Overview](http://duckdb.org/docs/stable/clients/overview)
        *   [Tertiary Clients](http://duckdb.org/docs/stable/clients/tertiary)
        *   [ADBC](http://duckdb.org/docs/stable/clients/adbc)
        *    C 

        
            *   [Overview](http://duckdb.org/docs/stable/clients/c/overview)
            *   [Startup](http://duckdb.org/docs/stable/clients/c/connect)
            *   [Configuration](http://duckdb.org/docs/stable/clients/c/config)
            *   [Query](http://duckdb.org/docs/stable/clients/c/query)
            *   [Data Chunks](http://duckdb.org/docs/stable/clients/c/data_chunk)
            *   [Vectors](http://duckdb.org/docs/stable/clients/c/vector)
            *   [Values](http://duckdb.org/docs/stable/clients/c/value)
            *   [Types](http://duckdb.org/docs/stable/clients/c/types)
            *   [Prepared Statements](http://duckdb.org/docs/stable/clients/c/prepared)
            *   [Appender](http://duckdb.org/docs/stable/clients/c/appender)
            *   [Table Functions](http://duckdb.org/docs/stable/clients/c/table_functions)
            *   [Replacement Scans](http://duckdb.org/docs/stable/clients/c/replacement_scans)
            *   [API Reference](http://duckdb.org/docs/stable/clients/c/api)

        *   [C++](http://duckdb.org/docs/stable/clients/cpp)
        *    CLI 

        
            *   [Overview](http://duckdb.org/docs/stable/clients/cli/overview)
            *   [Arguments](http://duckdb.org/docs/stable/clients/cli/arguments)
            *   [Dot Commands](http://duckdb.org/docs/stable/clients/cli/dot_commands)
            *   [Output Formats](http://duckdb.org/docs/stable/clients/cli/output_formats)
            *   [Editing](http://duckdb.org/docs/stable/clients/cli/editing)
            *   [Safe Mode](http://duckdb.org/docs/stable/clients/cli/safe_mode)
            *   [Autocomplete](http://duckdb.org/docs/stable/clients/cli/autocomplete)
            *   [Syntax Highlighting](http://duckdb.org/docs/stable/clients/cli/syntax_highlighting)
            *   [Known Issues](http://duckdb.org/docs/stable/clients/cli/known_issues)

        *   [Dart](http://duckdb.org/docs/stable/clients/dart)
        *   [Go](http://duckdb.org/docs/stable/clients/go)
        *   [Java (JDBC)](http://duckdb.org/docs/stable/clients/java)
        *   [Julia](http://duckdb.org/docs/stable/clients/julia)
        *    Node.js (Deprecated) 

        
            *   [Overview](http://duckdb.org/docs/stable/clients/nodejs/overview)
            *   [API Reference](http://duckdb.org/docs/stable/clients/nodejs/reference)

        *    Node.js (Neo) 

        
            *   [Overview](http://duckdb.org/docs/stable/clients/node_neo/overview)

        *    ODBC 

        
            *   [Overview](http://duckdb.org/docs/stable/clients/odbc/overview)
            *   [Linux Setup](http://duckdb.org/docs/stable/clients/odbc/linux)
            *   [Windows Setup](http://duckdb.org/docs/stable/clients/odbc/windows)
            *   [macOS Setup](http://duckdb.org/docs/stable/clients/odbc/macos)
            *   [Configuration](http://duckdb.org/docs/stable/clients/odbc/configuration)

        *   [PHP](http://duckdb.org/docs/stable/clients/php)
        *    Python 

        
            *   [Overview](http://duckdb.org/docs/stable/clients/python/overview)
            *   [Data Ingestion](http://duckdb.org/docs/stable/clients/python/data_ingestion)
            *   [Conversion between DuckDB and Python](http://duckdb.org/docs/stable/clients/python/conversion)
            *   [DB API](http://duckdb.org/docs/stable/clients/python/dbapi)
            *   [Relational API](http://duckdb.org/docs/stable/clients/python/relational_api)
            *   [Function API](http://duckdb.org/docs/stable/clients/python/function)
            *   [Types API](http://duckdb.org/docs/stable/clients/python/types)
            *   [Expression API](http://duckdb.org/docs/stable/clients/python/expression)
            *   [Spark API](http://duckdb.org/docs/stable/clients/python/spark_api)
            *   [API Reference](http://duckdb.org/docs/stable/clients/python/reference)
            *   [Known Python Issues](http://duckdb.org/docs/stable/clients/python/known_issues)

        *   [R](http://duckdb.org/docs/stable/clients/r)
        *   [Rust](http://duckdb.org/docs/stable/clients/rust)
        *   [Swift](http://duckdb.org/docs/stable/clients/swift)
        *    Wasm 

        
            *   [Overview](http://duckdb.org/docs/stable/clients/wasm/overview)
            *   [Deploying DuckDB-Wasm](http://duckdb.org/docs/stable/clients/wasm/deploying_duckdb_wasm)
            *   [Instantiation](http://duckdb.org/docs/stable/clients/wasm/instantiation)
            *   [Data Ingestion](http://duckdb.org/docs/stable/clients/wasm/data_ingestion)
            *   [Query](http://duckdb.org/docs/stable/clients/wasm/query)
            *   [Extensions](http://duckdb.org/docs/stable/clients/wasm/extensions)

    *    SQL 

    
        *   [Introduction](http://duckdb.org/docs/stable/sql/introduction)
        *    Statements 

        
            *   [Overview](http://duckdb.org/docs/stable/sql/statements/overview)
            *   [ANALYZE](http://duckdb.org/docs/stable/sql/statements/analyze)
            *   [ALTER TABLE](http://duckdb.org/docs/stable/sql/statements/alter_table)
            *   [ALTER VIEW](http://duckdb.org/docs/stable/sql/statements/alter_view)
            *   [ATTACH and DETACH](http://duckdb.org/docs/stable/sql/statements/attach)
            *   [CALL](http://duckdb.org/docs/stable/sql/statements/call)
            *   [CHECKPOINT](http://duckdb.org/docs/stable/sql/statements/checkpoint)
            *   [COMMENT ON](http://duckdb.org/docs/stable/sql/statements/comment_on)
            *   [COPY](http://duckdb.org/docs/stable/sql/statements/copy)
            *   [CREATE INDEX](http://duckdb.org/docs/stable/sql/statements/create_index)
            *   [CREATE MACRO](http://duckdb.org/docs/stable/sql/statements/create_macro)
            *   [CREATE SCHEMA](http://duckdb.org/docs/stable/sql/statements/create_schema)
            *   [CREATE SECRET](http://duckdb.org/docs/stable/sql/statements/create_secret)
            *   [CREATE SEQUENCE](http://duckdb.org/docs/stable/sql/statements/create_sequence)
            *   [CREATE TABLE](http://duckdb.org/docs/stable/sql/statements/create_table)
            *   [CREATE VIEW](http://duckdb.org/docs/stable/sql/statements/create_view)
            *   [CREATE TYPE](http://duckdb.org/docs/stable/sql/statements/create_type)
            *   [DELETE](http://duckdb.org/docs/stable/sql/statements/delete)
            *   [DESCRIBE](http://duckdb.org/docs/stable/sql/statements/describe)
            *   [DROP](http://duckdb.org/docs/stable/sql/statements/drop)
            *   [EXPORT and IMPORT DATABASE](http://duckdb.org/docs/stable/sql/statements/export)
            *   [INSERT](http://duckdb.org/docs/stable/sql/statements/insert)
            *   [LOAD / INSTALL](http://duckdb.org/docs/stable/sql/statements/load_and_install)
            *   [MERGE INTO](http://duckdb.org/docs/stable/sql/statements/merge_into)
            *   [PIVOT](http://duckdb.org/docs/stable/sql/statements/pivot)
            *   [Profiling](http://duckdb.org/docs/stable/sql/statements/profiling)
            *   [SELECT](http://duckdb.org/docs/stable/sql/statements/select)
            *   [SET / RESET](http://duckdb.org/docs/stable/sql/statements/set)
            *   [SET VARIABLE](http://duckdb.org/docs/stable/sql/statements/set_variable)
            *   [SHOW and SHOW DATABASES](http://duckdb.org/docs/stable/sql/statements/show)
            *   [SUMMARIZE](http://duckdb.org/docs/stable/sql/statements/summarize)
            *   [Transaction Management](http://duckdb.org/docs/stable/sql/statements/transactions)
            *   [UNPIVOT](http://duckdb.org/docs/stable/sql/statements/unpivot)
            *   [UPDATE](http://duckdb.org/docs/stable/sql/statements/update)
            *   [USE](http://duckdb.org/docs/stable/sql/statements/use)
            *   [VACUUM](http://duckdb.org/docs/stable/sql/statements/vacuum)

        *    Query Syntax 

        
            *   [SELECT](http://duckdb.org/docs/stable/sql/query_syntax/select)
            *   [FROM and JOIN](http://duckdb.org/docs/stable/sql/query_syntax/from)
            *   [WHERE](http://duckdb.org/docs/stable/sql/query_syntax/where)
            *   [GROUP BY](http://duckdb.org/docs/stable/sql/query_syntax/groupby)
            *   [GROUPING SETS](http://duckdb.org/docs/stable/sql/query_syntax/grouping_sets)
            *   [HAVING](http://duckdb.org/docs/stable/sql/query_syntax/having)
            *   [ORDER BY](http://duckdb.org/docs/stable/sql/query_syntax/orderby)
            *   [LIMIT and OFFSET](http://duckdb.org/docs/stable/sql/query_syntax/limit)
            *   [SAMPLE](http://duckdb.org/docs/stable/sql/query_syntax/sample)
            *   [Unnesting](http://duckdb.org/docs/stable/sql/query_syntax/unnest)
            *   [WITH](http://duckdb.org/docs/stable/sql/query_syntax/with)
            *   [WINDOW](http://duckdb.org/docs/stable/sql/query_syntax/window)
            *   [QUALIFY](http://duckdb.org/docs/stable/sql/query_syntax/qualify)
            *   [VALUES](http://duckdb.org/docs/stable/sql/query_syntax/values)
            *   [FILTER](http://duckdb.org/docs/stable/sql/query_syntax/filter)
            *   [Set Operations](http://duckdb.org/docs/stable/sql/query_syntax/setops)
            *   [Prepared Statements](http://duckdb.org/docs/stable/sql/query_syntax/prepared_statements)

        *    Data Types 

        
            *   [Overview](http://duckdb.org/docs/stable/sql/data_types/overview)
            *   [Array](http://duckdb.org/docs/stable/sql/data_types/array)
            *   [Bitstring](http://duckdb.org/docs/stable/sql/data_types/bitstring)
            *   [Blob](http://duckdb.org/docs/stable/sql/data_types/blob)
            *   [Boolean](http://duckdb.org/docs/stable/sql/data_types/boolean)
            *   [Date](http://duckdb.org/docs/stable/sql/data_types/date)
            *   [Enum](http://duckdb.org/docs/stable/sql/data_types/enum)
            *   [Interval](http://duckdb.org/docs/stable/sql/data_types/interval)
            *   [List](http://duckdb.org/docs/stable/sql/data_types/list)
            *   [Literal Types](http://duckdb.org/docs/stable/sql/data_types/literal_types)
            *   [Map](http://duckdb.org/docs/stable/sql/data_types/map)
            *   [NULL Values](http://duckdb.org/docs/stable/sql/data_types/nulls)
            *   [Numeric](http://duckdb.org/docs/stable/sql/data_types/numeric)
            *   [Struct](http://duckdb.org/docs/stable/sql/data_types/struct)
            *   [Text](http://duckdb.org/docs/stable/sql/data_types/text)
            *   [Time](http://duckdb.org/docs/stable/sql/data_types/time)
            *   [Timestamp](http://duckdb.org/docs/stable/sql/data_types/timestamp)
            *   [Time Zones](http://duckdb.org/docs/stable/sql/data_types/timezones)
            *   [Union](http://duckdb.org/docs/stable/sql/data_types/union)
            *   [Typecasting](http://duckdb.org/docs/stable/sql/data_types/typecasting)

        *    Expressions 

        
            *   [Overview](http://duckdb.org/docs/stable/sql/expressions/overview)
            *   [CASE Expression](http://duckdb.org/docs/stable/sql/expressions/case)
            *   [Casting](http://duckdb.org/docs/stable/sql/expressions/cast)
            *   [Collations](http://duckdb.org/docs/stable/sql/expressions/collations)
            *   [Comparisons](http://duckdb.org/docs/stable/sql/expressions/comparison_operators)
            *   [IN Operator](http://duckdb.org/docs/stable/sql/expressions/in)
            *   [Logical Operators](http://duckdb.org/docs/stable/sql/expressions/logical_operators)
            *   [Star Expression](http://duckdb.org/docs/stable/sql/expressions/star)
            *   [Subqueries](http://duckdb.org/docs/stable/sql/expressions/subqueries)
            *   [TRY](http://duckdb.org/docs/stable/sql/expressions/try)

        *    Functions 

        
            *   [Overview](http://duckdb.org/docs/stable/sql/functions/overview)
            *   [Aggregate Functions](http://duckdb.org/docs/stable/sql/functions/aggregates)
            *   [Array Functions](http://duckdb.org/docs/stable/sql/functions/array)
            *   [Bitstring Functions](http://duckdb.org/docs/stable/sql/functions/bitstring)
            *   [Blob Functions](http://duckdb.org/docs/stable/sql/functions/blob)
            *   [Date Format Functions](http://duckdb.org/docs/stable/sql/functions/dateformat)
            *   [Date Functions](http://duckdb.org/docs/stable/sql/functions/date)
            *   [Date Part Functions](http://duckdb.org/docs/stable/sql/functions/datepart)
            *   [Enum Functions](http://duckdb.org/docs/stable/sql/functions/enum)
            *   [Interval Functions](http://duckdb.org/docs/stable/sql/functions/interval)
            *   [Lambda Functions](http://duckdb.org/docs/stable/sql/functions/lambda)
            *   [List Functions](http://duckdb.org/docs/stable/sql/functions/list)
            *   [Map Functions](http://duckdb.org/docs/stable/sql/functions/map)
            *   [Nested Functions](http://duckdb.org/docs/stable/sql/functions/nested)
            *   [Numeric Functions](http://duckdb.org/docs/stable/sql/functions/numeric)
            *   [Pattern Matching](http://duckdb.org/docs/stable/sql/functions/pattern_matching)
            *   [Regular Expressions](http://duckdb.org/docs/stable/sql/functions/regular_expressions)
            *   [Struct Functions](http://duckdb.org/docs/stable/sql/functions/struct)
            *   [Text Functions](http://duckdb.org/docs/stable/sql/functions/text)
            *   [Time Functions](http://duckdb.org/docs/stable/sql/functions/time)
            *   [Timestamp Functions](http://duckdb.org/docs/stable/sql/functions/timestamp)
            *   [Timestamp with Time Zone Functions](http://duckdb.org/docs/stable/sql/functions/timestamptz)
            *   [Union Functions](http://duckdb.org/docs/stable/sql/functions/union)
            *   [Utility Functions](http://duckdb.org/docs/stable/sql/functions/utility)
            *   [Window Functions](http://duckdb.org/docs/stable/sql/functions/window_functions)

        *   [Constraints](http://duckdb.org/docs/stable/sql/constraints)
        *   [Indexes](http://duckdb.org/docs/stable/sql/indexes)
        *    Meta Queries 

        
            *   [Information Schema](http://duckdb.org/docs/stable/sql/meta/information_schema)
            *   [Metadata Functions](http://duckdb.org/docs/stable/sql/meta/duckdb_table_functions)

        *    DuckDB's SQL Dialect 

        
            *   [Overview](http://duckdb.org/docs/stable/sql/dialect/overview)
            *   [Indexing](http://duckdb.org/docs/stable/sql/dialect/indexing)
            *   [Friendly SQL](http://duckdb.org/docs/stable/sql/dialect/friendly_sql)
            *   [Keywords and Identifiers](http://duckdb.org/docs/stable/sql/dialect/keywords_and_identifiers)
            *   [Order Preservation](http://duckdb.org/docs/stable/sql/dialect/order_preservation)
            *   [PostgreSQL Compatibility](http://duckdb.org/docs/stable/sql/dialect/postgresql_compatibility)
            *   [SQL Quirks](http://duckdb.org/docs/stable/sql/dialect/sql_quirks)

        *   [Samples](http://duckdb.org/docs/stable/sql/samples)

    *    Configuration 

    
        *   [Overview](http://duckdb.org/docs/stable/configuration/overview)
        *   [Pragmas](http://duckdb.org/docs/stable/configuration/pragmas)
        *   [Secrets Manager](http://duckdb.org/docs/stable/configuration/secrets_manager)

    *    Extensions 

    
        *   [Overview](http://duckdb.org/docs/stable/extensions/overview)
        *   [Installing Extensions](http://duckdb.org/docs/stable/extensions/installing_extensions)
        *   [Advanced Installation Methods](http://duckdb.org/docs/stable/extensions/advanced_installation_methods)
        *   [Distributing Extensions](http://duckdb.org/docs/stable/extensions/extension_distribution)
        *   [Versioning of Extensions](http://duckdb.org/docs/stable/extensions/versioning_of_extensions)
        *   [Troubleshooting of Extensions](http://duckdb.org/docs/stable/extensions/troubleshooting)

    *    Core Extensions 

    
        *   [Overview](http://duckdb.org/docs/stable/core_extensions/overview)
        *   [AutoComplete](http://duckdb.org/docs/stable/core_extensions/autocomplete)
        *   [Avro](http://duckdb.org/docs/stable/core_extensions/avro)
        *   [AWS](http://duckdb.org/docs/stable/core_extensions/aws)
        *   [Azure](http://duckdb.org/docs/stable/core_extensions/azure)
        *   [Delta](http://duckdb.org/docs/stable/core_extensions/delta)
        *   [DuckLake](http://duckdb.org/docs/stable/core_extensions/ducklake)
        *   [Encodings](http://duckdb.org/docs/stable/core_extensions/encodings)
        *   [Excel](http://duckdb.org/docs/stable/core_extensions/excel)
        *   [Full Text Search](http://duckdb.org/docs/stable/core_extensions/full_text_search)
        *    httpfs (HTTP and S3) 

        
            *   [Overview](http://duckdb.org/docs/stable/core_extensions/httpfs/overview)
            *   [HTTP(S) Support](http://duckdb.org/docs/stable/core_extensions/httpfs/https)
            *   [Hugging Face](http://duckdb.org/docs/stable/core_extensions/httpfs/hugging_face)
            *   [S3 API Support](http://duckdb.org/docs/stable/core_extensions/httpfs/s3api)
            *   [Legacy Authentication Scheme for S3 API](http://duckdb.org/docs/stable/core_extensions/httpfs/s3api_legacy_authentication)

        *    Iceberg 

        
            *   [Overview](http://duckdb.org/docs/stable/core_extensions/iceberg/overview)
            *   [Iceberg REST Catalogs](http://duckdb.org/docs/stable/core_extensions/iceberg/iceberg_rest_catalogs)
            *   [Amazon S3 Tables](http://duckdb.org/docs/stable/core_extensions/iceberg/amazon_s3_tables)
            *   [Amazon SageMaker Lakehouse (AWS Glue)](http://duckdb.org/docs/stable/core_extensions/iceberg/amazon_sagemaker_lakehouse)
            *   [Troubleshooting](http://duckdb.org/docs/stable/core_extensions/iceberg/troubleshooting)

        *   [ICU](http://duckdb.org/docs/stable/core_extensions/icu)
        *   [inet](http://duckdb.org/docs/stable/core_extensions/inet)
        *   [jemalloc](http://duckdb.org/docs/stable/core_extensions/jemalloc)
        *   [MySQL](http://duckdb.org/docs/stable/core_extensions/mysql)
        *   [PostgreSQL](http://duckdb.org/docs/stable/core_extensions/postgres)
        *    Spatial 

        
            *   [Overview](http://duckdb.org/docs/stable/core_extensions/spatial/overview)
            *   [Function Reference](http://duckdb.org/docs/stable/core_extensions/spatial/functions)
            *   [R-Tree Indexes](http://duckdb.org/docs/stable/core_extensions/spatial/r-tree_indexes)
            *   [GDAL Integration](http://duckdb.org/docs/stable/core_extensions/spatial/gdal)

        *   [SQLite](http://duckdb.org/docs/stable/core_extensions/sqlite)
        *   [TPC-DS](http://duckdb.org/docs/stable/core_extensions/tpcds)
        *   [TPC-H](http://duckdb.org/docs/stable/core_extensions/tpch)
        *   [UI](http://duckdb.org/docs/stable/core_extensions/ui)
        *   [Unity Catalog](http://duckdb.org/docs/stable/core_extensions/unity_catalog)
        *   [Vortex](http://duckdb.org/docs/stable/core_extensions/vortex)
        *   [VSS](http://duckdb.org/docs/stable/core_extensions/vss)

    *    Guides 

    
        *   [Overview](http://duckdb.org/docs/stable/guides/overview)
        *    Data Viewers 

        
            *   [Tableau](http://duckdb.org/docs/stable/guides/data_viewers/tableau)
            *   [CLI Charting with YouPlot](http://duckdb.org/docs/stable/guides/data_viewers/youplot)

        *    Database Integration 

        
            *   [Overview](http://duckdb.org/docs/stable/guides/database_integration/overview)
            *   [MySQL Import](http://duckdb.org/docs/stable/guides/database_integration/mysql)
            *   [PostgreSQL Import](http://duckdb.org/docs/stable/guides/database_integration/postgres)
            *   [SQLite Import](http://duckdb.org/docs/stable/guides/database_integration/sqlite)

        *    File Formats 

        
            *   [Overview](http://duckdb.org/docs/stable/guides/file_formats/overview)
            *   [CSV Import](http://duckdb.org/docs/stable/guides/file_formats/csv_import)
            *   [CSV Export](http://duckdb.org/docs/stable/guides/file_formats/csv_export)
            *   [Directly Reading Files](http://duckdb.org/docs/stable/guides/file_formats/read_file)
            *   [Excel Import](http://duckdb.org/docs/stable/guides/file_formats/excel_import)
            *   [Excel Export](http://duckdb.org/docs/stable/guides/file_formats/excel_export)
            *   [JSON Import](http://duckdb.org/docs/stable/guides/file_formats/json_import)
            *   [JSON Export](http://duckdb.org/docs/stable/guides/file_formats/json_export)
            *   [Parquet Import](http://duckdb.org/docs/stable/guides/file_formats/parquet_import)
            *   [Parquet Export](http://duckdb.org/docs/stable/guides/file_formats/parquet_export)
            *   [Querying Parquet Files](http://duckdb.org/docs/stable/guides/file_formats/query_parquet)
            *   [File Access with the file: Protocol](http://duckdb.org/docs/stable/guides/file_formats/file_access)

        *    Network and Cloud Storage 

        
            *   [Overview](http://duckdb.org/docs/stable/guides/network_cloud_storage/overview)
            *   [HTTP Parquet Import](http://duckdb.org/docs/stable/guides/network_cloud_storage/http_import)
            *   [S3 Parquet Import](http://duckdb.org/docs/stable/guides/network_cloud_storage/s3_import)
            *   [S3 Parquet Export](http://duckdb.org/docs/stable/guides/network_cloud_storage/s3_export)
            *   [S3 Iceberg Import](http://duckdb.org/docs/stable/guides/network_cloud_storage/s3_iceberg_import)
            *   [S3 Express One](http://duckdb.org/docs/stable/guides/network_cloud_storage/s3_express_one)
            *   [GCS Import](http://duckdb.org/docs/stable/guides/network_cloud_storage/gcs_import)
            *   [Cloudflare R2 Import](http://duckdb.org/docs/stable/guides/network_cloud_storage/cloudflare_r2_import)
            *   [DuckDB over HTTPS / S3](http://duckdb.org/docs/stable/guides/network_cloud_storage/duckdb_over_https_or_s3)
            *   [Fastly Object Storage Import](http://duckdb.org/docs/stable/guides/network_cloud_storage/fastly_object_storage_import)

        *    Meta Queries 

        
            *   [Describe Table](http://duckdb.org/docs/stable/guides/meta/describe)
            *   [EXPLAIN: Inspect Query Plans](http://duckdb.org/docs/stable/guides/meta/explain)
            *   [EXPLAIN ANALYZE: Profile Queries](http://duckdb.org/docs/stable/guides/meta/explain_analyze)
            *   [List Tables](http://duckdb.org/docs/stable/guides/meta/list_tables)
            *   [Summarize](http://duckdb.org/docs/stable/guides/meta/summarize)
            *   [DuckDB Environment](http://duckdb.org/docs/stable/guides/meta/duckdb_environment)

        *    ODBC 

        
            *   [ODBC Guide](http://duckdb.org/docs/stable/guides/odbc/general)

        *    Performance 

        
            *   [Overview](http://duckdb.org/docs/stable/guides/performance/overview)
            *   [Environment](http://duckdb.org/docs/stable/guides/performance/environment)
            *   [Import](http://duckdb.org/docs/stable/guides/performance/import)
            *   [Schema](http://duckdb.org/docs/stable/guides/performance/schema)
            *   [Indexing](http://duckdb.org/docs/stable/guides/performance/indexing)
            *   [Join Operations](http://duckdb.org/docs/stable/guides/performance/join_operations)
            *   [File Formats](http://duckdb.org/docs/stable/guides/performance/file_formats)
            *   [How to Tune Workloads](http://duckdb.org/docs/stable/guides/performance/how_to_tune_workloads)
            *   [My Workload Is Slow](http://duckdb.org/docs/stable/guides/performance/my_workload_is_slow)
            *   [Benchmarks](http://duckdb.org/docs/stable/guides/performance/benchmarks)
            *   [Working with Huge Databases](http://duckdb.org/docs/stable/guides/performance/working_with_huge_databases)

        *    Python 

        
            *   [Installation](http://duckdb.org/docs/stable/guides/python/install)
            *   [Executing SQL](http://duckdb.org/docs/stable/guides/python/execute_sql)
            *   [Jupyter Notebooks](http://duckdb.org/docs/stable/guides/python/jupyter)
            *   [marimo Notebooks](http://duckdb.org/docs/stable/guides/python/marimo)
            *   [SQL on Pandas](http://duckdb.org/docs/stable/guides/python/sql_on_pandas)
            *   [Import from Pandas](http://duckdb.org/docs/stable/guides/python/import_pandas)
            *   [Export to Pandas](http://duckdb.org/docs/stable/guides/python/export_pandas)
            *   [Import from Numpy](http://duckdb.org/docs/stable/guides/python/import_numpy)
            *   [Export to Numpy](http://duckdb.org/docs/stable/guides/python/export_numpy)
            *   [SQL on Arrow](http://duckdb.org/docs/stable/guides/python/sql_on_arrow)
            *   [Import from Arrow](http://duckdb.org/docs/stable/guides/python/import_arrow)
            *   [Export to Arrow](http://duckdb.org/docs/stable/guides/python/export_arrow)
            *   [Relational API on Pandas](http://duckdb.org/docs/stable/guides/python/relational_api_pandas)
            *   [Multiple Python Threads](http://duckdb.org/docs/stable/guides/python/multiple_threads)
            *   [Integration with Ibis](http://duckdb.org/docs/stable/guides/python/ibis)
            *   [Integration with Polars](http://duckdb.org/docs/stable/guides/python/polars)
            *   [Using fsspec Filesystems](http://duckdb.org/docs/stable/guides/python/filesystems)

        *    SQL Editors 

        
            *   [DBeaver SQL IDE](http://duckdb.org/docs/stable/guides/sql_editors/dbeaver)

        *    SQL Features 

        
            *   [AsOf Join](http://duckdb.org/docs/stable/guides/sql_features/asof_join)
            *   [Full-Text Search](http://duckdb.org/docs/stable/guides/sql_features/full_text_search)
            *   [Graph Queries](http://duckdb.org/docs/stable/guides/sql_features/graph_queries)
            *   [query and query_table Functions](http://duckdb.org/docs/stable/guides/sql_features/query_and_query_table_functions)
            *   [Merge Statement for SCD Type 2](http://duckdb.org/docs/stable/guides/sql_features/merge)
            *   [Timestamp Issues](http://duckdb.org/docs/stable/guides/sql_features/timestamps)

        *    Snippets 

        
            *   [Creating Synthetic Data](http://duckdb.org/docs/stable/guides/snippets/create_synthetic_data)
            *   [Dutch Railway Datasets](http://duckdb.org/docs/stable/guides/snippets/dutch_railway_datasets)
            *   [Sharing Macros](http://duckdb.org/docs/stable/guides/snippets/sharing_macros)
            *   [Analyzing a Git Repository](http://duckdb.org/docs/stable/guides/snippets/analyze_git_repository)
            *   [Importing Duckbox Tables](http://duckdb.org/docs/stable/guides/snippets/importing_duckbox_tables)
            *   [Copying an In-Memory Database to a File](http://duckdb.org/docs/stable/guides/snippets/copy_in-memory_database_to_file)

        *    Troubleshooting 

        
            *   [Crashes](http://duckdb.org/docs/stable/guides/troubleshooting/crashes)
            *   [Out of Memory Errors](http://duckdb.org/docs/stable/guides/troubleshooting/oom_errors)

        *   [Glossary of Terms](http://duckdb.org/docs/stable/guides/glossary)
        *   [Browsing Offline](http://duckdb.org/docs/stable/guides/offline-copy)

    *    Operations Manual 

    
        *   [Overview](http://duckdb.org/docs/stable/operations_manual/overview)
        *    DuckDB's Footprint 

        
            *   [Files Created by DuckDB](http://duckdb.org/docs/stable/operations_manual/footprint_of_duckdb/files_created_by_duckdb)
            *   [Gitignore for DuckDB](http://duckdb.org/docs/stable/operations_manual/footprint_of_duckdb/gitignore_for_duckdb)
            *   [Reclaiming Space](http://duckdb.org/docs/stable/operations_manual/footprint_of_duckdb/reclaiming_space)

        *    Installing DuckDB 

        
            *   [Install Script](http://duckdb.org/docs/stable/operations_manual/installing_duckdb/install_script)

        *    Logging 

        
            *   [Overview](http://duckdb.org/docs/stable/operations_manual/logging/overview)

        *    Securing DuckDB 

        
            *   [Overview](http://duckdb.org/docs/stable/operations_manual/securing_duckdb/overview)
            *   [Embedding DuckDB](http://duckdb.org/docs/stable/operations_manual/securing_duckdb/embedding_duckdb)
            *   [Securing Extensions](http://duckdb.org/docs/stable/operations_manual/securing_duckdb/securing_extensions)

        *   [Non-Deterministic Behavior](http://duckdb.org/docs/stable/operations_manual/non-deterministic_behavior)
        *   [Limits](http://duckdb.org/docs/stable/operations_manual/limits)
        *   [DuckDB Docker Container](http://duckdb.org/docs/stable/operations_manual/duckdb_docker)

    *    Development 

    
        *   [DuckDB Repositories](http://duckdb.org/docs/stable/dev/repositories)
        *   [Release Cycle](http://duckdb.org/docs/stable/dev/release_cycle)
        *   [Profiling](http://duckdb.org/docs/stable/dev/profiling)
        *    Building DuckDB 

        
            *   [Overview](http://duckdb.org/docs/stable/dev/building/overview)
            *   [Build Configuration](http://duckdb.org/docs/stable/dev/building/build_configuration)
            *   [Building Extensions](http://duckdb.org/docs/stable/dev/building/building_extensions)
            *   [Android](http://duckdb.org/docs/stable/dev/building/android)
            *   [Linux](http://duckdb.org/docs/stable/dev/building/linux)
            *   [macOS](http://duckdb.org/docs/stable/dev/building/macos)
            *   [Raspberry Pi](http://duckdb.org/docs/stable/dev/building/raspberry_pi)
            *   [Windows](http://duckdb.org/docs/stable/dev/building/windows)
            *   [Python](http://duckdb.org/docs/stable/dev/building/python)
            *   [R](http://duckdb.org/docs/stable/dev/building/r)
            *   [Troubleshooting](http://duckdb.org/docs/stable/dev/building/troubleshooting)
            *   [Unofficial and Unsupported Platforms](http://duckdb.org/docs/stable/dev/building/unofficial_and_unsupported_platforms)

        *   [Benchmark Suite](http://duckdb.org/docs/stable/dev/benchmark)
        *    Testing 

        
            *   [Overview](http://duckdb.org/docs/stable/dev/sqllogictest/overview)
            *   [sqllogictest Introduction](http://duckdb.org/docs/stable/dev/sqllogictest/intro)
            *   [Writing Tests](http://duckdb.org/docs/stable/dev/sqllogictest/writing_tests)
            *   [Debugging](http://duckdb.org/docs/stable/dev/sqllogictest/debugging)
            *   [Result Verification](http://duckdb.org/docs/stable/dev/sqllogictest/result_verification)
            *   [Persistent Testing](http://duckdb.org/docs/stable/dev/sqllogictest/persistent_testing)
            *   [Loops](http://duckdb.org/docs/stable/dev/sqllogictest/loops)
            *   [Multiple Connections](http://duckdb.org/docs/stable/dev/sqllogictest/multiple_connections)
            *   [Catch](http://duckdb.org/docs/stable/dev/sqllogictest/catch)

    *    Internals 

    
        *   [Overview](http://duckdb.org/docs/stable/internals/overview)
        *   [Storage Versions and Format](http://duckdb.org/docs/stable/internals/storage)
        *   [Execution Format](http://duckdb.org/docs/stable/internals/vector)
        *   [Pivot](http://duckdb.org/docs/stable/internals/pivot)

*   [Sitemap](http://duckdb.org/docs/sitemap)
*   [Live Demo](https://shell.duckdb.org/)

Documentation/Client APIs/CLI

Syntax Highlighting

> Syntax highlighting in the CLI is currently only available for macOS and Linux.

SQL queries that are written in the shell are automatically highlighted using syntax highlighting.

![Image 3: Image showing syntax highlighting in the shell](http://duckdb.org/images/syntax_highlighting_screenshot.png)

There are several components of a query that are highlighted in different colors. The colors can be configured using [dot commands](http://duckdb.org/docs/stable/clients/cli/dot_commands.html). Syntax highlighting can also be disabled entirely using the 
```plaintext
.highlight off
```
 command.

Below is a list of components that can be configured.

| Type | Command | Default color |
| --- | --- | --- |
| Keywords | ```plaintext .keyword ``` | ```plaintext green ``` |
| Constants and literals | ```plaintext .constant ``` | ```plaintext yellow ``` |
| Comments | ```plaintext .comment ``` | ```plaintext brightblack ``` |
| Errors | ```plaintext .error ``` | ```plaintext red ``` |
| Continuation | ```plaintext .cont ``` | ```plaintext brightblack ``` |
| Continuation (Selected) | ```plaintext .cont_sel ``` | ```plaintext green ``` |

The components can be configured using either a supported color name (e.g., 
```plaintext
.keyword red
```
), or by directly providing a terminal code to use for rendering (e.g., 
```plaintext
.keywordcode \033[31m
```
). Below is a list of supported color names and their corresponding terminal codes.

| Color | Terminal code |
| --- | --- |
| red | ```plaintext \033[31m ``` |
| green | ```plaintext \033[32m ``` |
| yellow | ```plaintext \033[33m ``` |
| blue | ```plaintext \033[34m ``` |
| magenta | ```plaintext \033[35m ``` |
| cyan | ```plaintext \033[36m ``` |
| white | ```plaintext \033[37m ``` |
| brightblack | ```plaintext \033[90m ``` |
| brightred | ```plaintext \033[91m ``` |
| brightgreen | ```plaintext \033[92m ``` |
| brightyellow | ```plaintext \033[93m ``` |
| brightblue | ```plaintext \033[94m ``` |
| brightmagenta | ```plaintext \033[95m ``` |
| brightcyan | ```plaintext \033[96m ``` |
| brightwhite | ```plaintext \033[97m ``` |

For example, here is an alternative set of syntax highlighting colors:

```
.keyword brightred
.constant brightwhite
.comment cyan
.error yellow
.cont blue
.cont_sel brightblue
```

If you wish to start up the CLI with a different set of colors every time, you can place these commands in the 
```plaintext
~/.duckdbrc
```
 file that is loaded on start-up of the CLI.

[Error Highlighting](http://duckdb.org/docs/stable/clients/cli/syntax_highlighting.html#error-highlighting)
-----------------------------------------------------------------------------------------------------------

The shell has support for highlighting certain errors. In particular, mismatched brackets and unclosed quotes are highlighted in red (or another color if specified). This highlighting is automatically disabled for large queries. In addition, it can be disabled manually using the 
```plaintext
.render_errors off
```
 command.

##### About this page

*   [Report content issue](https://github.com/duckdb/duckdb-web/issues/new?title=Issue%20found%20on%20page%20%27Syntax%20Highlighting%27&labels=issue%20found%20on%20page&body=%0A%3E%20Please%20describe%20the%20problem%20you%20encountered%20in%20the%20DuckDB%20documentation%20and%20include%20the%20%22Page%20URL%22%20link%20shown%20below.%0A%3E%20Note:%20only%20create%20an%20issue%20if%20you%20wish%20to%20report%20a%20problem%20with%20the%20DuckDB%20documentation.%20For%20questions%20about%20DuckDB%20or%20the%20use%20of%20certain%20DuckDB%20features,%20use%20[GitHub%20Discussions](https://github.com/duckdb/duckdb/discussions/),%20[Stack%20Overflow](https://stackoverflow.com/questions/tagged/duckdb),%20or%20[Discord](https://discord.duckdb.org/).%0A%0APage%20URL:%20%3Chttps://duckdb.org/docs/stable/clients/cli/syntax_highlighting.html%3E%0A "Create GitHub issue")
*   [See this page as Markdown](https://raw.githubusercontent.com/duckdb/duckdb-web/refs/heads/main/docs/stable/clients/cli/syntax_highlighting.md "See Markdown")
*   [Edit this page on GitHub](https://github.com/duckdb/duckdb-web/edit/main/docs/stable/clients/cli/syntax_highlighting.md "Go to GitHub")

© 2025 DuckDB Foundation, Amsterdam NL

[Code of Conduct](http://duckdb.org/code_of_conduct.html)[Trademark Use](http://duckdb.org/trademark_guidelines.html)

##### In this article

*   [Error Highlighting](http://duckdb.org/docs/stable/clients/cli/syntax_highlighting.html#error-highlighting)

----
## Notes / Comments / Lessons

- Scope: stable docs only, focused on selected client APIs plus overview pages.
- Conversion path: r.jina.ai markdown proxy.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
