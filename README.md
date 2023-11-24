# git-credential-github-apps

a custom git credential helper using GitHub Apps

```
[credential "https://github.com"]
	helper = !git-credential-github-apps --app-id <ID> --private-key <PEM FILE>
	useHttpPath = true
```
