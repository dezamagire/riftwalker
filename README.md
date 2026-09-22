# Riftwalker

> A minimal Windows workspace bar inspired by Waybar and Noctalia.

Riftwalker is a lightweight desktop bar for Windows built around **Windows Virtual Desktops**. It provides workspace switching, system information, appearance customization, and foreground application tracking in a compact interface designed to stay out of the way.

The goal is simple: bring some of the workflow and visual language of Linux desktop bars to Windows without trying to turn Windows into a fake tiling window manager.

## Screenshots

> Screenshots coming soon.

## Features

* **Virtual desktop workspaces**

  * Switch between Windows Virtual Desktops
  * Workspace indicators for desktops 1–4
  * Global `Win + 1` through `Win + 4` shortcuts

* **Foreground application**

  * Detects the currently focused Windows application
  * Displays the active application in the center of the bar

* **Theme system**

  * Built-in Void theme
  * Multiple selectable color themes
  * Dedicated theme picker
  * Live theme switching

* **Appearance controls**

  * Adjustable border width
  * Adjustable border radius
  * Adjustable background opacity

* **System integration**

  * Windows AppBar integration
  * Always-on-top behavior
  * Non-activating bar
  * Windows-native virtual desktop APIs

## Planned

Riftwalker is still in active development. Planned features include:

* Spotify currently-playing track
* CPU and RAM monitoring
* Clock and system status
* More themes
* Custom theme configuration
* Additional workspace controls
* Improved multi-monitor support
* Configurable keyboard shortcuts
* Persistent user configuration
* Installer and release builds

The feature list will probably change. This is software development, after all. Requirements have a fascinating habit of appearing after the code does.

## Tech Stack

* **Tauri 2**
* **Rust**
* **TypeScript**
* **React**
* **Vite**
* **Bun**
* **Windows API**

Tauri provides the desktop shell and native system integration while React handles the interface. Rust is used for Windows-specific functionality such as virtual desktop management, keyboard hooks, AppBar registration, and foreground-window detection. Tauri is designed around using a system webview rather than bundling a separate browser engine with the application.

## Development

### Requirements

Riftwalker currently targets **Windows**.

You will need:

* Windows 10/11
* Rust with the MSVC toolchain
* Bun
* Tauri CLI
* Microsoft C++ Build Tools
* WebView2

Tauri's Windows development environment requires Microsoft C++ Build Tools and WebView2.

### Clone

```powershell
git clone https://github.com/YOUR_USERNAME/riftwalker.git
cd riftwalker
```

### Install dependencies

```powershell
bun install
```

### Run in development

```powershell
bun tauri dev
```

### Build

```powershell
bun tauri build
```

Tauri can produce Windows installers such as `.msi` and NSIS setup executables through its bundling system.

## Project Structure

```text
riftwalker/
├── src/
│   ├── components/
│   ├── styles/
│   ├── App.tsx
│   └── index.css
│
├── src-tauri/
│   ├── src/
│   │   ├── appbar.rs
│   │   ├── desktops.rs
│   │   ├── foreground.rs
│   │   ├── hotkeys.rs
│   │   └── lib.rs
│   ├── capabilities/
│   └── tauri.conf.json
│
├── package.json
├── bun.lock
└── README.md
```

## Design

Riftwalker intentionally follows a restrained visual direction:

* dark interfaces
* minimal ornamentation
* monochrome foundations
* selective green and purple accents
* compact typography
* technical rather than "gamer" aesthetics

The interface is inspired by tools such as **Waybar** and **Noctalia**, but Riftwalker is built specifically around the Windows desktop environment.

## Status

**Early development**

The core workspace bar and Windows Virtual Desktop integration are functional. The project is currently focused on expanding system information, customization, and desktop integrations before a first public release.

Expect rough edges. There are currently several of them. They are being domesticated.

## License

License information will be added before the first public release.
