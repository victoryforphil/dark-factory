----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/config/template
- Keywords: moon, moonrepo, docs, monorepo, build, config, template
- Summary: The `template.yml` file configures metadata and variables for a template,
----

Source: https://moonrepo.dev/docs/config/template

# template

The `template.yml` file configures metadata and variables for a template,
[used by the generator](/docs/guides/codegen), and must exist at the root of a named template folder.

## `id`v1.23.0

Overrides the name (identifier) of the template, instead of inferring the name from the template
folder. Be aware that template names must be unique across the workspace, and across all template
locations that have been configured in [`generator.templates`](/docs/config/workspace#templates).

template.yml

```
id: 'npm-package'
```

## `title`Required

A human readable title that will be displayed during the [`moon generate`](/docs/commands/generate)
process.

template.yml

```
title: 'npm package'
```

## `description`Required

A description of why the template exists, what its purpose is, and any other relevant information.

template.yml

```
description: |  Scaffolds the initial structure for an npm package,  including source and test folders, a package.json, and more.
```

## `destination`v1.19.0

An optional file path in which this template should be generated into. This provides a mechanism for
standardizing a destination location, and avoids having to manually pass a destination to
[`moon generate`](/docs/commands/generate).

If the destination is prefixed with `/`, it will be relative from the workspace root, otherwise it
is relative from the current working directory.

template.yml

```
destination: 'packages/[name]'
```

This setting supports [template variables](#variables) through `[varName]` syntax. Learn more in
the [code generation documentation](/docs/guides/codegen#interpolation).

## `extends`v1.19.0

One or many other templates that this template should extend. Will deeply inherit all template files
and variables.

template.yml

```
extends: ['base', 'configs']
```

## `variables`

A mapping of variables that will be interpolated into all template files and file system paths when
[rendering with Tera](https://tera.netlify.app/docs/#variables). The map key is the variable name
(in camelCase or snake_case), while the value is a configuration object, as described with the
settings below.

template.yml

```
variables:  name:    type: 'string'    default: ''    required: true    prompt: 'Package name?'
```

### `type`Required

The type of value for the variable. Accepts `array`, `boolean`, `string`, `object`, `number`, or
`enum`. Floats are not supported, use strings instead.

For arrays and objects, the value of each member must be a JSON compatible type.

### `internal`v1.23.0

Marks a variable as internal only, which avoids the variable value being overwritten by command line
arguments.

### `order`v1.23.0

The order in which the variable will be prompted to the user. By default, variables are prompted in
the order they are defined in the `template.yml` file.

### Primitives & collections

Your basic primitives: boolean, numbers, strings, and collections: arrays, objects.

- array
- boolean
- number
- object
- string

template.yml

```
variables:  type:    type: 'array'    prompt: 'Type?'    default: ['app', 'lib']
```

template.yml

```
variables:  private:    type: 'boolean'    prompt: 'Private?'    default: false
```

template.yml

```
variables:  age:    type: 'number'    prompt: 'Age?'    default: 0    required: true
```

template.yml

```
variables:  metadata:    type: 'object'    prompt: 'Metadata?'    default:      type: 'lib'      dev: true
```

template.yml

```
variables:  name:    type: 'string'    prompt: 'Name?'    required: true
```

### `default`Required

The default value of the variable. When `--defaults` is passed to
[`moon generate`](/docs/commands/generate) or [`prompt`](#prompt) is not defined, the default value
will be used, otherwise the user will be prompted to enter a custom value.

### `prompt`

When defined, will prompt the user with a message in the terminal to input a custom value, otherwise
[`default`](#default) will be used.

For arrays and objects, a valid JSON string must be provided as the value.

### `required`

Marks the variable as required during prompting only. For arrays, strings, and objects, will error
for empty values (`''`). For numbers, will error for zero's (`0`).

### Enums

An enum is an explicit list of string values that a user can choose from.

template.yml

```
variables:  color:    type: 'enum'    values: ['red', 'green', 'blue', 'purple']    default: 'purple'    prompt: 'Favorite color?'
```

### `default`

The default value of the variable. When `--defaults` is passed to
[`moon generate`](/docs/commands/generate) or [`prompt`](#prompt) is not defined, the default value
will be used, otherwise the user will be prompted to enter a custom value.

For enums, the default value can be a string when [`multiple`](#multiple) is false, or a string or
an array of strings when `multiple` is true. Furthermore, each default value must exist in the
[`values`](#values) list.

template.yml

```
# Singlevariables:  color:    type: 'enum'    values: ['red', 'green', 'blue', 'purple']    default: 'purple'    prompt: 'Favorite color?'# Multiplevariables:  color:    type: 'enum'    values: ['red', 'green', 'blue', 'purple']    default: ['red', 'purple']    multiple: true    prompt: 'Favorite color?'
```

### `prompt`

When defined, will prompt the user with a message in the terminal to input a custom value, otherwise
[`default`](#default) will be used.

### `multiple`

Allows multiple values to be chosen during prompting. In the template, an array or strings will be
rendered, otherwise when not-multiple, a single string will be.

### `values`Required

List of explicit values to choose from. Can either be defined with a string, which acts as a value
and label, or as an object, which defines an explicit value and label.

template.yml

```
variables:  color:    type: 'enum'    values:      - 'red'      # OR      - value: 'red'        label: 'Red 🔴'    # ...
```

## Frontmatter

The following settings are not available in `template.yml`, but can be defined as frontmatter at
the top of a template file. View the [code generation guide](/docs/guides/codegen#frontmatter) for more
information.

### `force`

When enabled, will always overwrite a file of the same name at the destination path, and will bypass
any prompting in the terminal.

```
---force: true---Some template content!
```

### `to`

Defines a custom file path, relative from the destination root, in which to create the file. This
will override the file path within the template folder, and allow for conditional rendering and
engine filters to be used.

```
{% set component_name = name | pascal_case %}---to: components/{{ component_name }}.tsx---export function {{ component_name }}() {  return
;}
```

### `skip`

When enabled, the template file will be skipped while writing to the destination path. This setting
can be used to conditionally render a file.

```
---skip: {{ name == "someCondition" }}---Some template content!
```

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
