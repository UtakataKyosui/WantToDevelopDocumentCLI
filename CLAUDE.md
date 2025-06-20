# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

WTD (Want-Driven Development) is a Rust CLI tool that helps developers manage project ideas and documentation using a template-based approach. The tool allows users to:

- Create projects from customizable templates that include "Want" documents (project ideas/requirements) and development documentation
- Manage multiple projects with selection and status tracking
- Generate structured project directories with templated content

## Common Commands

### Build and Run
```bash
cargo build
cargo run -- <subcommand>
```

### Development Commands
```bash
# Run the CLI with different subcommands
cargo run -- init "My Project"
cargo run -- select "My Project"
cargo run -- status
cargo run -- setup

# Build for release
cargo build --release
```

### Testing
```bash
cargo test
cargo check
```

## Architecture Overview

The project follows a modular CLI architecture:

### Core Modules
- **main.rs**: Entry point with error handling and command routing
- **parser.rs**: CLI argument parsing using clap with subcommands
- **commands.rs**: Implementation of core CLI operations (Init, Select, Status) following the `Runnable` trait pattern
- **config.rs**: Configuration management with JSON persistence to user config directory
- **template.rs**: Template system using Tera for rendering and YAML for metadata
- **setup.rs**: Initial setup creating default templates in user config directory

### Key Concepts

**Template System**: Templates are stored in `~/.config/wtd/templates/` and consist of:
- `template.yaml`: Metadata defining template name, description, and file structure
- Template files (like `want.md`, `docs/api-design.md`) using Tera templating syntax
- Variables available in templates: `{{title}}`, `{{slug}}`, `{{date}}`, `{{project}}`

**Project Structure**: Generated projects follow the pattern:
```
<output_dir>/WTD/<project_name>/
├── Want/
│   └── <project-slug>.md
└── Develop-Docs/
    └── <various>.md
```

**Configuration**: User config stored at `~/.config/wtd/config.json` with:
- `selected_project`: Currently active project
- `default_output_dir`: Where to create new projects
- `default_template`: Template to use by default
- `author`: Author name for templates

### Error Handling
- Uses `anyhow` for error propagation throughout
- Japanese error messages in main.rs error handler
- Context is added to errors using `.with_context()` pattern

### Dependencies
- `clap`: CLI argument parsing with derive macros
- `tera`: Template rendering engine
- `serde`/`serde_json`/`serde_yaml`: Serialization
- `dialoguer`: Interactive prompts for template selection
- `dirs`: Cross-platform config directory detection
- `slug`: URL-safe project name generation
- `chrono`: Date generation for templates

## Development Notes

- The codebase uses Japanese comments and error messages
- Template commands (TemplateList, TemplateNew, TemplateDelete) are marked as `todo!()` and not yet implemented
- The `Runnable` trait provides a consistent interface for all CLI commands
- Templates use the Tera templating engine with variables injected via Context