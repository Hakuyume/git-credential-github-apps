# git-credential-github-apps

a custom git credential helper using GitHub Apps

```
[credential "https://github.com"]
	helper = !git-credential-github-apps --app-id-from-literal <ID> --private-key-from-file <PEM>
	useHttpPath = true
```
