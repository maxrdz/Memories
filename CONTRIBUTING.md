# Contributing Guidelines

If you are new to the GNOME community and software ecosystem,
the Gallery project would like to give you a warm welcome to
the GNOME community. We highly encourage you to visit
[welcome.gnome.org](https://welcome.gnome.org/), a website
made by the GNOME foundation to introduce new users to GNOME.

Users interested in making contributions to the software
are encouraged to check out
[developer.gnome.org](https://developer.gnome.org/).

Before starting to write your own contribution, please make
sure to read the project [README](./README) beforehand!

## Git Commit Naming Convention

Git commit messages should follow the guidelines for commit
messages provided in the
[GNOME Handbook](https://handbook.gnome.org/development/commit-messages.html).

The following is the structure of a commit message, along
with an example of a standard commit message.

`<directory>: <summary>` e.g. "po: Updated Mexican Spanish translation"

We also encourage to provide a more elaborate description of
your changes inside your commit description.

```
po: Updated Mexican Spanish translation

Update the translation for "Library", which was previously
"Biblioteca" to "Fototeca". This word is unique to the
language and is a more suitable translation for a library
of photos, not books!

Closes #1234
```

"As a general rule, it is always better to write too much in the
commit message body than too little."
(GNOME/[gnome-shell](https://gitlab.gnome.org/GNOME/gnome-shell/-/blob/9f5a323e06d6b5b3818d934ba5b31c437c4c07b3/docs/commit-messages.md))

