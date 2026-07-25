## [unreleased]

### 🚀 Features

- *(task-crud-basic)* Add task completion
- *(task-crud-basic)* Add task deletion
- *(task-crud-basic)* Add basic, hardcoded task addition
- *(task-crud-basic)* Implement 2nd screen for adding tasks via user input
- *(task-crud-basic)* Allow escaping without adding a task
- *(task-crud-basic)* Draw the add task popup on top of the list
- *(task-crud-basic)* Center the add task popup

### 🐛 Bug Fixes

- *(task-crud-basic)* Fix bug where task list index remains at the last deleted task
- *(task-crud-basic)* Fix the popup still having text after exiting
- *(task-crud-basic)* Clear the area of the block before rendering the popup
- *(task-crud-basic)* Use ratatui Layout instead of half baked math for the area calculation

### ⚙️ Miscellaneous Tasks

- Add GitHub Actions pipeline for testing and releases
- Update todo housekeeping
- *(task-crud-basic)* Make the adding task title more descriptive

### 💡 Quality of Life

- *(task-crud-basic)* Add `with_` methods to `Task`
- *(task-crud-basic)* Derive Default on Task

### 💼 Other

- Implement core task creation, deletion, completion, and popup UI
## [0.1.0] - 2026-07-20

### 🚀 Features

- *(tasks)* Add basic task struct
- *(tasks)* Add widget rendering for tasks
- *(tasks)* Use `tui_widget_list::ListView` for rendering widgets
- *(tasks)* Add focusing of tasks and basic focused task styles
- *(tasks)* Add strikethrough of completed tasks
- *(tasks)* Add line wrapping
- *(tasks)* Improve line wrapping

### 🐛 Bug Fixes

- *(tasks)* Subtract the width of borders and checkbox to get actual width for wrapping
- *(tasks)* Fix off-by-one in width comparison
- *(tasks)* Make 0-width title vector empty;
- *(tasks)* Make empty title have at least a height of 3
- *(tasks)* Use saturating sub in actual width calculation
- *(lint)* Apply cargo clippy fixes

### 🚜 Refactor

- *(tasks)* Move task rendering to a TaskWidget with an immutable ref to a task
- *(tasks)* Use horizontal![] instead of constraints![]
- *(tasks)* Move setting is_focused to a setter function
- *(tasks)* Rename border width constant to DUAL_BORDER_SIZE

### 🎨 Styling

- *(tasks)* Clean up styling logic

### 🧪 Testing

- *(tasks)* Add a test for line wrapping
- *(tasks)* Add extensive testing for line wrapping

### ⚙️ Miscellaneous Tasks

- *(readme)* Patch up the readme to not make it look like a ghost town

### 💼 Other

- *(cargo-toml)* Add cargo package metadata;
