# Contributing Guidelines

If you are new to the GNOME community and software ecosystem,
the Album project would like to give you a warm welcome to
the GNOME community. We highly encourage you to visit
[welcome.gnome.org](https://welcome.gnome.org/), a website
made by the GNOME foundation to introduce new users to GNOME.

Users interested in making contributions to the software
behind Album are encouraged to check out
[developer.gnome.org](https://developer.gnome.org/).

Before starting to write your own contribution, please make
sure to read the project [README](./README) beforehand!

## Getting Started

The most important first step is to read the
[GNOME Code of Conduct](https://conduct.gnome.org/).

If you are planning on contributing to the translation work,
please see [GNOME Damned Lies](https://l10n.gnome.org/).

- If you don't yet have an account with GNOME's GitLab instance,
please [register on GitLab](https://gitlab.gnome.org/users/sign_up),
this will be important for submitting your changes!
- Set up your [SSH](https://en.wikipedia.org/wiki/Secure_Shell)
cryptographic key in your GNOME GitLab account.
Alternatively, you can also register a
[PGP](https://en.wikipedia.org/wiki/Pretty_Good_Privacy)
cryptographic key for signing your commits within Git.
[welcome.gnome.org](https://welcome.gnome.org/en/app/Loupe/#setting-up-gitlab)
has a good tutorial on setting up your GNOME GitLab account.
- Once you have your account setup and signed in,
[fork the project Git repository](https://gitlab.gnome.org/maxrdz/Album).
This will create your own copy of the source under your user's namespace.
- Next, **clone** your new fork on your local machine. If you have your
SSH key configured on your GNOME GitLab account, run the following:
```sh
git clone git@ssh.gitlab.gnome.org:maxrdz/Album.git
```
If you do not have SSH set up, you can also clone via HTTPS.
```sh
git clone https://gitlab.gnome.org/maxrdz/Album.git
```
- Before starting to write your contribution, create a **new branch**
for your patch/contribution within Git:
```sh
git checkout -b new-patch
```
- After writing your changes, **commit** your changes:
```sh
git commit -am 'src: Fixed bug and resolved #1234'
```
and **push** your new branch to your remote fork.
```sh
git push origin new-patch
```
- The remote Git server at GNOME GitLab may recognize that
you pushed to your fork and will send you a message containing
a link to create a **new merge request** on the GitLab website.
If you do not receive this message by any chance, there is also
a guide by GNOME on creating a new merge request at
[welcome.gnome.org](https://welcome.gnome.org/en/app/Loupe/#creating-a-merge-request).

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

