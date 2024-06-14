# git-credential-github-apps

a custom git credential helper using GitHub Apps

```console
$ git config credential.https://github.com/.helper 'github-apps --app-id-from-literal=<APP ID> --private-key-from-file=<PRIVATE KEY>'
$ git config credential.https://github.com/.useHttpPath true
$ git config url.https://github.com/.insteadOf git@github.com:  # optional
```
