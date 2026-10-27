# Sito Node Ports

App de barra de menús (Tauri 2 + React + `@sito/ui`) que lista los procesos Node
(`node`, `bun`, `deno`) con puertos TCP en escucha.

- Primer arranque: una ventana pide permiso para instalarse en la barra de menús.
- Con permiso: la app arranca oculta, sin icono en el Dock. El número junto al
  icono = procesos escuchando.
- Click en el icono: cada proceso aparece como `label · :puertos`. El submenú
  abre `http://localhost:<puerto>`, muestra script, PID y carpeta, y permite
  terminarlo (SIGTERM).
- **Label** = `name` del `package.json` más cercano al cwd del proceso; si no
  hay, el nombre de la carpeta.

Datos: `lsof -iTCP -sTCP:LISTEN`, `lsof -d cwd` y `ps`. Solo ve procesos de tu
usuario.

## Desarrollo

```sh
pnpm install
pnpm tauri dev
```

Para volver a ver la pantalla de permiso: "Quitar de la barra de menús" en la
ventana, o borrar `~/Library/Application Support/com.sito8943.node-ports/settings.json`.

## Instalar (Homebrew)

```sh
brew install --cask sito8943/tap/sito-node-ports
xattr -dr com.apple.quarantine "/Applications/Sito Node Ports.app"
```

La app no está firmada: el segundo comando evita que macOS bloquee el primer
arranque.

## Publicar una versión

1. Sube la versión en `package.json`, `src-tauri/Cargo.toml` y
   `src-tauri/tauri.conf.json`.
2. Publica un Release `vX.Y.Z` en GitHub. Los workflows construyen los `.dmg`
   (Intel y Apple Silicon), los adjuntan y actualizan el cask en
   `sito8943/homebrew-tap`.

Requisito único: secreto `TAP_GITHUB_TOKEN` en el repo (PAT con Contents:write
sobre `sito8943/homebrew-tap`).
