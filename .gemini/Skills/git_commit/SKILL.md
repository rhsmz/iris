---
name: git_commit
description: Semantic Commit Message (English), feature branches by phase, and PR creation workflow skill.
---

# Git Commit Skill

## Overview

In this project, **Semantic Commit Message** is the basis, and all commit messages are written in **English**. (Note: Original rule said Japanese, translating to English as requested).
In addition, implementation is done by creating feature branches for each function/phase and merging them into `main` through PRs (Pull Requests).

---

## 1. Commit Message Conventions

### Basic Format

```
<type>(<scope>): <summary>

[Body (optional)]

[Footer (optional)]
```

### List of types

| type       | Use Case                                           |
|------------|----------------------------------------------------|
| `feat`     | Addition of a new feature                          |
| `fix`      | Bug fix                                            |
| `refactor` | Refactoring without changing behavior             |
| `perf`     | Performance improvement                            |
| `test`     | Addition or modification of tests                  |
| `docs`     | Changes only to documents/comments                 |
| `style`    | Formatting, whitespace, semicolons, etc. (no logic change) |
| `build`    | Changes to build system/dependencies (Cargo.toml, etc.) |
| `ci`       | Changes to CI/CD settings                          |
| `chore`    | Other maintenance work                             |
| `revert`   | Reverting a previous commit                        |

### List of scopes (This Project)

| scope           | Target Crate / Module                             |
|-----------------|---------------------------------------------------|
| `engine_core`   | `crates/engine_core` overall                      |
| `script_editor` | `crates/script_editor` overall                    |
| `game_player`   | `crates/game_player` overall                      |
| `ecs`           | ECS infrastructure modules                        |
| `scene`         | Scene management modules                          |
| `render`        | Rendering engine (Filament integration)           |
| `audio`         | Sound system                                      |
| `asset`         | Asset management                                  |
| `script`        | Script parser/State machine                       |
| `ui`            | UI layout/rendering                               |
| `live2d`        | Live2D Cubism SDK integration                     |
| `vrm`           | VRM/FBX model integration                         |
| `save`          | Save/Load system                                  |
| `minigame`      | Minigame plugin                                   |
| `workspace`     | Cargo workspace/Root settings                     |
| `agent`         | Rules, skills, and workflows under `.gemini/`      |

### Commit Message Examples

```
feat(ecs): Define basic types for World, Entity, and Component
fix(scene): Fix issue where assets are not released after on_exit
refactor(render): Extract CommandBuffer generation process into a function
docs(agent): Add git_commit skill
build(workspace): Add engine_core crate to Cargo.toml
test(save): Add round-trip test for save/load
```

### How to Write Body and Footer

- Describe **why** the change was made in the body if it cannot be conveyed by the summary alone.
- For breaking changes, include a line starting with `BREAKING CHANGE:` in the body.
- Describe related Issues in the footer as `Closes #<Number>`.

```
feat(script): Implement jump command for tag-based script

Added goto/gosub commands necessary for branching production.
Manage the return destination of gosub using the stack of the state machine.

Closes #42
```

---

## 2. Feature Branch Strategy

### Branch Naming Convention

```
feature/<phase>/<task-description>
```

| Element              | Description                                                                     |
|----------------------|---------------------------------------------------------------------------------|
| `<phase>`            | Phase number + sub-phase letter (e.g., `p1a`, `p1b`, `p2a`, `p2b`)              |
| `<task-description>` | English kebab-case string representing the feature concisely (granularity of 1 PR) |

**Examples:**

```
feature/p1a/cargo-workspace-init
feature/p1b/gitignore-rustfmt-config
feature/p2a/ecs-world-entity-types
feature/p2b/ecs-component-storage
feature/p2c/ecs-system-scheduler
feature/p3a/scene-trait-definition
feature/p3b/scene-manager-stack
feature/p3c/scene-async-transition
```

### Branch Creation Procedures

```bash
# Derive from the latest main
git switch main
git pull origin main
git switch -c feature/<phase>/<task-description>
```

### Guidelines for Commit Granularity

- 1 commit = **One logically coherent change** (type definition only, implementation only, test only, etc.)
- Commit in a state where the build passes (passing `cargo check`)
- Target **3 to 10 commits** per PR (complete within a functional unit)
- Manage WIP commits in progress with `git commit --fixup` or `git stash`, and organize with `rebase -i` before PR.

---

## 3. PR (Pull Request) Creation Procedures

### PR Title

Write in the same format as the commit message.

```
feat(ecs): Implement World, Entity, and Component for ECS infrastructure
```

### PR Body Template

```markdown
## Overview

<!-- What you did in this PR, why it is necessary -->

## Changes

- [ ] Item 1
- [ ] Item 2

## Test Method

```bash
cargo test -p engine_core
```

## Related Issue

Closes #<Number>
```

### PR Creation using GitHub CLI

```bash
# Push feature branch
git push -u origin feature/<phase>/<task-description>

# Create PR (gh command)
gh pr create \
  --title "feat(ecs): Implement World, Entity, and Component for ECS infrastructure" \
  --body-file .gemini/skills/git_commit/pr_template.md \
  --base main \
  --head feature/<phase>/<task-description>
```

---

## 4. Branch Planning by Phase

Subdivide each phase into functional sub-branches (a/b/c...).
1 sub-branch = 1 PR = complete in one function/responsibility.

### P1: Workspace Initialization

| Branch                                | Main Content                                      |
|---------------------------------------|---------------------------------------------------|
| `feature/p1a/cargo-workspace-init`    | Root `Cargo.toml`, workspace member definitions   |
| `feature/p1b/gitignore-rustfmt`       | `.gitignore`, `rustfmt.toml`, `clippy.toml`       |
| `feature/p1c/crate-stub-modules`      | Creation of `lib.rs` / `main.rs` stubs for each crate |

### P2: ECS Infrastructure

| Branch                                | Main Content                                      |
|---------------------------------------|---------------------------------------------------|
| `feature/p2a/ecs-world-entity`        | Type definitions for `World`, `Entity` (ID generation/reuse) |
| `feature/p2b/ecs-component-storage`   | `ComponentStorage` (SparseSet / Dense Array)      |
| `feature/p2c/ecs-system-scheduler`    | `System` trait, scheduler, execution order control |
| `feature/p2d/ecs-query-api`           | `Query<T>` / `QueryMut<T>` Query API              |
| `feature/p2e/ecs-event-bus`           | Event queue, `EventReader` / `EventWriter`        |

### P3: Scene Management

| Branch                                | Main Content                                      |
|---------------------------------------|---------------------------------------------------|
| `feature/p3a/scene-trait-definition`  | `Scene` trait, `SceneTransition` enum             |
| `feature/p3b/scene-manager-stack`     | `SceneManager` (stack management, push/pop/replace) |
| `feature/p3c/scene-context`           | `SceneContext` (message passing infrastructure)   |
| `feature/p3d/scene-async-transition`  | Asynchronous transition, loading screen            |

### P4: Asset Management

| Branch                                | Main Content                                      |
|---------------------------------------|---------------------------------------------------|
| `feature/p4a/asset-descriptor`        | `AssetDescriptor`, asset identifier/type definition |
| `feature/p4b/asset-loader-core`       | `AssetLoader` trait, synchronous load infrastructure |
| `feature/p4c/asset-async-loading`     | Asynchronous load, progress notification          |
| `feature/p4d/asset-cache`             | Asset cache, reference count/release               |

### P5: Rendering Engine (Filament)

| Branch                                | Main Content                                      |
|---------------------------------------|---------------------------------------------------|
| `feature/p5a/filament-ffi-bindings`   | Filament C++ -> Rust FFI bindings                 |
| `feature/p5b/render-command-buffer`   | `CommandBuffer`, drawing command queue             |
| `feature/p5c/render-multi-view`       | Game view (Layer 0) + UI view (Layer 1)           |
| `feature/p5d/render-pbr-materials`    | PBR material, IBL settings                        |
| `feature/p5e/render-post-process`     | Post-process (bloom, tone mapping, etc.)          |

### P6: Sound System

| Branch                                | Main Content                                      |
|---------------------------------------|---------------------------------------------------|
| `feature/p6a/audio-backend-kira`      | `kira` integration, AudioManager initialization   |
| `feature/p6b/audio-bgm-player`        | BGM playback/crossfade                            |
| `feature/p6c/audio-se-voice`          | SE / Voice channel, mixer                         |

### P7: Script Engine

| Branch                                | Main Content                                      |
|---------------------------------------|---------------------------------------------------|
| `feature/p7a/script-tokenizer`        | Tag-based tokenizer, lexical analysis             |
| `feature/p7b/script-parser-ast`       | Parser, AST definition                            |
| `feature/p7c/script-state-machine`    | State machine, instruction execution loop         |
| `feature/p7d/script-commands-basic`   | Basic commands (text, wait, jump, label)          |
| `feature/p7e/script-commands-adv`     | Advanced commands (branch, call, gosub, variable) |

### P8: UI System

| Branch                                | Main Content                                      |
|---------------------------------------|---------------------------------------------------|
| `feature/p8a/ui-taffy-layout`         | Taffy Flexbox layout engine integration           |
| `feature/p8b/ui-signal-reactive`      | Signal-driven reactive system                     |
| `feature/p8c/ui-sdf-text`             | SDF text rendering                                |
| `feature/p8d/ui-glassmorphism`        | Glassmorphism material, blur effect               |
| `feature/p8e/ui-animation`            | UI animation, transitions                         |

### P9: Live2D Integration

| Branch                                | Main Content                                      |
|---------------------------------------|---------------------------------------------------|
| `feature/p9a/live2d-ffi-core`         | Cubism Core C++ FFI, model loading                |
| `feature/p9b/live2d-render-texture`   | Render-to-Texture pipeline                        |
| `feature/p9c/live2d-motion`           | Motion playback/blending                          |
| `feature/p9d/live2d-lipsync`          | Lip-sync (audio analysis -> parameter reflection) |

### P10: VRM/FBX Integration

| Branch                                | Main Content                                      |
|---------------------------------------|---------------------------------------------------|
| `feature/p10a/vrm-loader`             | VRM/GLB loading, `gltf` crate integration         |
| `feature/p10b/vrm-spring-bone`        | SpringBone physics simulation                     |
| `feature/p10c/vrm-blend-shape`        | BlendShape / MorphTarget control                  |
| `feature/p10d/fbx-import`             | FBX -> GLB conversion pipeline                    |

### P11: Save System

| Branch                                | Main Content                                      |
|---------------------------------------|---------------------------------------------------|
| `feature/p11a/save-data-schema`       | Save data struct, `serde` implementation          |
| `feature/p11b/save-msgpack-io`        | MessagePack (`rmp-serde`) serialization/IO        |
| `feature/p11c/save-migration`         | Version migration mechanism                       |
| `feature/p11d/save-slot-manager`      | Slot management, thumbnails/metadata              |

### P12: Minigame Plugin

| Branch                                | Main Content                                      |
|---------------------------------------|---------------------------------------------------|
| `feature/p12a/minigame-plugin-trait`  | `MinigamePlugin` trait, registration/call API      |
| `feature/p12b/minigame-2d-action`     | 2D action minigame prototype                      |
| `feature/p12c/minigame-rpg-battle`    | RPG battle minigame prototype                     |
| `feature/p12d/minigame-puzzle`        | Puzzle minigame prototype                         |
| `feature/p12e/minigame-rts`           | RTS minigame prototype                           |

---

## 5. Pre-commit Checklist

The agent verifies the following before executing a commit:

- [ ] Formatted with `cargo fmt --all`
- [ ] No errors or warnings with `cargo clippy --all-targets -- -D warnings`
- [ ] No build errors with `cargo check --all-targets`
- [ ] `cargo test` passes if relevant tests exist
- [ ] Commit message conforms to the format of this skill
- [ ] Scope is set appropriately

---

## 6. Best Practices

- **atomic commits**: Do not include multiple unrelated changes in 1 commit.
- **rebase before PR**: If main has advanced, update with `git rebase main` before moving to PR.
- **squash with caution**: Maintain meaningful commit history. Prioritize `Rebase and merge` over `Squash and merge` when merging PRs.
- **draft PR**: Utilize Draft PR if review is needed even during implementation.
- **conflict resolution**: Always check conflicts manually and do not depend on automatic merging.
