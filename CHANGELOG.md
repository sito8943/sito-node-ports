# Changelog

All notable changes to this project are documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [0.3.0] - 2026-10-27

### Added

- Escape hides the window when the menu bar tray is enabled.

## [0.2.0] - 2026-10-27

### Added

- Global ⌃⌥N shortcut and app reopen (Spotlight, Launchpad, Finder) show the
  window when the menu bar icon is hidden on small screens.
- Node.js app icon and monochrome template tray icon.
- English and Spanish UI for the window and tray menu, picked from the
  system's preferred languages.
- MIT license and Node.js non-affiliation notice.

## [0.1.0] - 2026-10-27

### Added

- Initial menu bar app (Tauri 2 + React + @sito/ui) that, after asking for
  consent, lists Node/Bun/Deno processes with listening TCP ports, labelled by
  their `package.json` name.
- macOS DMG builds and Homebrew cask publishing workflows.

[0.3.0]: https://github.com/sito8943/sito-node-ports/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/sito8943/sito-node-ports/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/sito8943/sito-node-ports/releases/tag/v0.1.0
