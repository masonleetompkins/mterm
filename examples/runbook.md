---
mterm:
  permissions:
    shell: ["git status", "git log --oneline -10", "echo *", "ls *"]
    allow_unspecified: prompt
---

# Demo Runbook

This is a plain markdown file that `mterm` can execute.

> [!BUTTON] Check Git Status

```sh :run id=check
git status
```

> [!BUTTON] Recent Commits

```sh :run id=log
git log --oneline -10
```

Regular code block (not runnable, just docs):

```sh
echo "this won't get a Run button"
```

Runnable without button (gets gutter Run):

```sh :run id=hello
echo "hello from mterm"
ls -la
```

- [ ] Verify deploy
- [ ] Check logs

> [!NOTE]
> Open this in `nvim` or GitHub and it still reads as normal markdown.
