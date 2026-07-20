# TODO.md

## Features

### Critical (v0.2 + v0.3)
- [ ] **CRUD:** Create, Update & Complete, and Delete tasks in memory. *Might require an ID system to accurately select tasks*
- [ ] **Serialization:** Make our Task struct serializable, save and load tasks from disk. Follow XDG Base directory specification.`~/.todo-rs` is not allowed.

### Could have
- [ ] **Theme:** Make a `struct Theme { ... }` and use it to make the program look good
- [ ] **Sorting** *(Once the task properties below are **sorted out**, for now completed below unfinished)*
- [ ] **Keybinds:** Show keybidns at the bottom of the screen
- [ ] **Config:** Might include save location, keybinds, theme
- [ ] **Undo/Redo**
- [ ] **Task grouping** (by due date)

### Housekeeping
- [ ] **CI/CD:**
  - [ ] Automatic building
  - [ ] Testing
  - [ ] Code linting
  - [ ] Github Release

## Task Properties

### Done
- [X] Title
- [X] Is done

### Main (v0.4)
- [ ] Priorities
- [ ] Due date

### Extra
- [ ] Category

## Done
- [X] Add tasks
- [X] Add line wrapping
- [X] Make rendering resistant to width related edge cases
- [X] Cargo.toml metadata
- [X] Fix clippy errors
