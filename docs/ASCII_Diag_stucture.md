# Project Structure

```text
.
├── .github/
│   └── workflows/
│       └── build-window.yml        # GitHub Actions: build cargo run --bin window        get artifact window.exe
│
├── docs/
│   └── ASCII_Diag_stucture.txt     # This file: project structure overview
│
├── src/
│   ├── bin/
│   │   ├── window.rs               # Bevy window entry point (desktop build)
│   │   └── cli_tictactoe.rs        # Bevy cli entry point (desktop build)
│   │
│   ├── app_state.rs                # AppState enum: MainMenu, InGame, GameOver
│   ├── config.rs                   # Global config (if used)
│   ├── lib.rs                      # Library root: exposes core, board, input, etc.
│   │
│   ├── core/                       # Pure game logic (no Bevy dependencies)
│   │   ├── board.rs                # CoreBoard: NxN board, get/set/place_mark
│   │   ├── player.rs               # CorePlayer: X / O
│   │   └── rules.rs                # check_winner, draw detection, core rules
│   │
│   ├── fps/                        # Optional FPS limiter / diagnostics
│   │   ├── mod.rs                  # FpsPlugin or feature wiring
│   │   └── systems.rs              # Systems related to frame pacing / diagnostics
│   │
│   ├── input/                      # Input layer: raw input → events
│   │   ├── events.rs               # CellClickedEvent (Message)
│   │   ├── systems_mouse.rs        # ButtonInput + camera → CellClickedEvent
│   │   ├── systems_intents_spawn.rs    # CellClickedEvent → CellClickIntent (entity)
│   │   ├── systems_intents_apply.rs    # Apply CellClickIntent → BoardState / CurrentPlayer
│   │   └── mod.rs                  # InputPlugin: registers messages & systems
│   │
│   ├── intent/                     # Intent layer abstractions (optional separation)
│   │   ├── mod.rs                  # Intent plugin / wiring (future)
│   │   └── traits.rs               # Shared traits / helpers for intent processing
│   │
│   ├── board/                      # Gameplay layer: board state & core plugin
│   │   ├── components.rs           # Cell, CellOwner, GameRoot, BoardRoot, etc.
│   │   ├── resources.rs            # BoardState, CurrentPlayer, GameConfig, GameResult
│   │   ├── traits.rs               # ApplyCellMove, GetCellOwner, SwitchPlayer
│   │   ├── systems_state.rs        # reset_game_state and related state systems
│   │   ├── systems_setup.rs        # setup_camera, setup_board, grid spawning
│   │   ├── systems_cleanup.rs      # cleanup_board, despawn entities on exit
│   │   └── mod.rs                  # CoreGamePlugin: wires gameplay & input pipeline
│   │
│   ├── ui/                         # UI / rendering layer (visuals on top of board)
│   │   ├── sprites/                # Visual representation of X / O marks
│   │   │   ├── components.rs       # CellVisual, MarkSprite and related UI components
│   │   │   ├── sprite_factory.rs   # Helper/factory for spawning X/O sprites
│   │   │   ├── systems_render.rs   # Reacts to BoardState changes and updates visuals
│   │   │   └── mod.rs              # Sprite-related UI module glue
│   │   └── mod.rs                  # UiPlugin (menus / HUD / overlays)
│   │
│   ├── localization/               # Localization system (text, languages)
│   │   ├── mod.rs                  # LocalizationPlugin wiring
│   │   ├── resources.rs            # Localization tables, current language, etc.
│   │   ├── systems_load.rs         # Load localization data from assets/config
│   │   └── systems_apply.rs        # Apply localized strings to UI elements
│   │
│   ├── settings/                   # Game settings (audio, graphics, controls)
│   │   ├── mod.rs                  # SettingsPlugin wiring
│   │   ├── resources.rs            # Settings data model (volumes, toggles, etc.)
│   │   ├── systems_load.rs         # Load settings from file / defaults
│   │   └── systems_save.rs         # Save settings back to disk
│   │
│   ├── sounds/                     # Sound system wiring (code side)
│   │   ├── mod.rs                  # SoundPlugin: registers audio systems
│   │   ├── resources.rs            # Handles to loaded AudioSource assets, sound bank
│   │   ├── systems_save.rs         # Not have realized yet
│   │   └── systems_load.rs         # Load sound assets via AssetServer
│   │
│   ├── debug_tools/                # Debugging, logging, overlays
│   │   ├── logging/
│   │   │   ├── resources.rs        # LogConfig, LogRuntimeState, LogEvent
│   │   │   └── systems.rs          # init_log_directory, flush_log_buffer
│   │   ├── systems_hotkeys.rs      # Debug hotkeys stub
│   │   ├── systems_overlay.rs      # Debug overlay stub
│   │   └── mod.rs                  # DebugToolsPlugin: wires logging & debug systems
│   │
│   └── utils.rs                    # (optional / future) shared helpers
│
├── assets/                         # Runtime assets loaded by AssetServer
│   ├── sounds/                     # Audio files (sfx, music)
│   ├── textures/                   # Sprites, UI textures (future)
│   └── fonts/                      # Fonts for UI (future)
│
├── .gitignore                      # Git ignore rules
├── Cargo.toml                      # Rust package manifest
├── Cargo.lock                      # Lockfile with exact dependency versions
├── LICENSE                         # MIT license
└── README.md                       # Project overview
```
