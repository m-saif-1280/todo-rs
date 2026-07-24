# TODO.md

## Features

### Critical (Minimum viable product)
- [ ] **Serialization:** Make our Task struct serializable, save and load tasks from disk. Follow XDG Base directory specification.`~/.todo-rs` is not allowed.

### Should Have
- [ ] Make the add task popup not look like a void box with input labels and showing the cursor

### Could have
- [ ] **Theme:** Make a `struct Theme { ... }` and use it to make the program look good
- [ ] **Sorting** *(Once the task properties below are **sorted out**, for now completed below unfinished)*
- [ ] **Keybinds:** Show keybidns at the bottom of the screen
- [ ] **Config:** Might include save location, keybinds, theme
- [ ] **Undo/Redo**
- [ ] **Task grouping** (by due date)

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
- [X] Create, Update & Complete, and Delete tasks in memory.
- [X] Add tasks
- [X] Add line wrapping
- [X] Make rendering resistant to width related edge cases
- [X] Cargo.toml metadata
- [X] Fix clippy errors
- [X] Add `with_*` methods to `Task` and derive `Default`
- [X] **CI/CD:**
