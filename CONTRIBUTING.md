# Contributing

First off, thank you for considering contributing to `todo-rs`!

## Creating an issue

Have an idea? Create an issue.
- [ ] Use the appropriate template for the appropriate request
- [ ] Fill out everything
- [ ] Use a descriptive title
- [ ] Don't make an issue only because you *can*

## 🔧 Submitting Code

Look for issues labeled `good first issue` or `help wanted` for
great places to start.

## Development Setup

### Prerequisites

- Rust **v1.96+**
- For the people who don't know, you also need **git.**

## Getting Started

```bash
# 1. Fork the repository on GitHub

# 2. Clone your fork locally
git clone https://github.com/m-saif-1280/todo-rs.git
cd todo-rs

# 3. Add upstream remote
# HTTPS, Use if you don't know what SSH is or don't care to use it
# or if you enjoy typing your PAT on every commit
git remote add github https://github.com/m-saif-1280/todo-rs.git

# SSH. Use if you already have the keys setup
git remote add github git@github.com:m-saif-1280/todo-rs.git

# 4. Install dependencies
cargo build

# 5. Create a branch for your changes
git checkout -b feat/my-feature
```

## Submitting a PR

### Before Submitting

1. **Update your branch:**
   ```bash
   git fetch github
   git rebase github/main
   ```

2. **Run all tests:**
   ```bash
   cargo test
   ```

3. **Format and lint:**
   ```bash
   cargo fmt
   cargo clippy -- -D warnings  # No warnings allowed!!!
   ```

4. **Update documentation** if you've changed APIs or added features.

### Submitting

1. Push your branch to your fork:
   ```bash
   git push github feat/my-feature
   ```

2. Open a Pull Request against `main`.

3. Wait for review.

## Commit Messages

- Use [conventional commits](https://conventionalcommits.org/)
- Atomic commits. NO "fixed ui" with 1132 changes made to files
- Descriptive commits. NO "fix bug", "pls work", "make fast", etc!

## Testing

- All new features must include tests
- Bug fixes should include regression tests
- Tests should be deterministic (no flaky tests)

---

Thank you for taking the time to contribute!
