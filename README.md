# git-credential-github-apps

a custom git credential helper using GitHub Apps

```
[credential "https://github.com"]
	helper = !git-credential-github-apps --app-id-from-literal <APP ID> --private-key-from-file <PRIVATE KEY FILE>
	useHttpPath = true
```
