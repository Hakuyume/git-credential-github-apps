# git-credential-github-apps

a custom git credential helper using GitHub Apps

```ini
[credential "https://github.com"]
	helper = git-credential-github-apps --app-id-from-literal=<APP ID> --private-key-from-file=<PRIVATE KEY>
	useHttpPath = true

[url "https://github.com/"]
	insteadOf = git@github.com:
```

```yaml
- uses: Hakuyume/git-credential-github-apps@main
  with:
    app-id: <APP ID>
    private-key: <PRIVATE KEY>
- run: git ...
```
