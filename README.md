# Tom Language

Tom Language is a lightweight Visual Studio Code extension that provides syntax highlighting for Taiwa Object Module (TOM) source files.

## Features
- Rich TextMate grammar for TOM keywords, types, classes, functions, variables, constants, operators, and preprocessor directives.
- Scoped highlighting inside strings, comments, and generic parameter lists.
- Comment TODO markers and emphasized assignment targets to surface common review items.

## File Associations
- `*.toms` (TOM source files)

## Language Overview
TOM (Taiwa Object Module) is used to customize Jissun Boushi CAD software. The language resembles C-style languages and is organised around classes, members, and executable blocks.

- Modules contain classes, and classes contain member functions, variables, and constants.
- Key module declarations include `title`, `author`, `native`, and `attach`.
- Classes may be marked `public`, `private`, `final`, or `abstract` and can derive from base classes with `extends`.
- Member variables and constants follow C-style declarations, while member functions define access modifiers, storage modifiers, and parameter lists.
- Comments support single line (`//`) and multi-line (`/* ... */`) forms.

Understanding these conventions helps when validating the highlighted scopes produced by the grammar.

## Usage
1. Install the extension from the Visual Studio Code Marketplace (search for **Tom Language**).
2. Open a `.toms` file. VS Code automatically associates the file with the TOM language ID.
3. Leverage TOM-aware syntax coloring and highlighting while editing

## Development
This repository contains only declarative contributions (grammar and configuration). If you modify the grammar:

1. Update `syntaxes/tom.tmLanguage.json`.
2. Reload VS Code with the `Developer: Reload Window` command to test changes.
3. Bump the version in `package.json` and update `CHANGELOG.md` before publishing.

## Resources
- [CHANGELOG](./CHANGELOG.md) for release history.
- [vsc-extension-quickstart](./vsc-extension-quickstart.md) for VS Code extension development tips.