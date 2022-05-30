# Dotium
Dotium is the provider of uniform Mod hosting platform objects, that get traits in libraries like ferinth and furse. These uniform objects are then used in Libraries like Libium to threat these Platform independently.

### Structure
#### There are 2 main components in Dotium:
- `modpacks` is used to define Modpacks
- `mods` are, who guessed it, mod structs. 


<details>
  <summary>
  <b>Full Gdevs Library Structure</b> (click to expand)
  </summary>
  <a href="https://github.com">Original</a> -
  <a href="https://github.com">Compact version</a>

```
  ╔════════╗ ╔════════╗     | Interfaces for Libium, both as CLI as GUI.
  ║ Carbon ║ ║ Ferium ║     | Ferium: CLI version written using Clap.
  ╚════╤═══╝ ╚═══╤════╝     | Carbon: GUI version using electron, written in SolidJS
       │         │
       ╰────┬────╯
            │
       ╔════╧════╗          | Libium, the library that does all the platform independent
       ║ Libium  ║          | work, both for Mod loaders as for Mod hosting Platforms.
       ╚════╤════╝          | Manages profiles, launches the game, modifies the config...
            │
╭───────────╯
│
│ ┏━━━━━━━━━━━━━━━━━━━━━━┓  | Extendable Mod loaders, managing Minecraft's inner game
├─┨     Mod loaders      ┃  | files, like metadata, versions and launch commands.
│ ┃╔═══════╗   ╔════════╗┃  |
│ ┃║ Faber ║∙∙∙║ Forgic ║┃  | Faber: Manager for the Fabric Mod Loader
│ ┃╚═══════╝   ╚════════╝┃  | Forgic: Manager for the Forge Mod Loader
│ ┗━━━━━━━━━━┯━━━━━━━━━━━┛
│        ╔═══╧═══╗          | Ludic, the library providing uniform Mod loader objects,
│        ║ Ludic ║          | that get traits in the Mod loader implementations.
│        ╚═══════╝          |
│
│ ┏━━━━━━━━━━━━━━━━━━━━━━━┓ | Extendable Mod hosting platforms, providing everything from
╰─┨      Platforms        ┃ | Mods, Resource Packs, Modpacks and Worlds to Datapacks,
  ┃╔═══════╗   ╔═════════╗┃ | Server Plugins and Shaders.
  ┃║ Furse ║∙∙∙║ Ferinth ║┃ |
  ┃╚═══════╝   ╚═════════╝┃ | Furse: Worker for the CurseForge API
  ┗━━━━━━━━━━┯━━━━━━━━━━━━┛ | Ferinth: Implementation for Modrinth
             │
        ╔════╧════╗         | Dotium, providing uniform Platform objects that then get traits
        ║ Dotium  ║         | in the Platform implementations.
        ╚═════════╝         |
```
</details>

### Development
- Compiling <br/>
  `cargo build`
- Testing <br/>
  `cargo test`
