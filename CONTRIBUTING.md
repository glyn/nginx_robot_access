# Contributing to nginx_robot_access

The nginx_robot_access project team welcomes contributions from the community.

## Contribution flow

This is a rough outline of what a contributor's workflow looks like:

- Create a topic branch from where you want to base your work
- Make commits of logical units
- Make sure your commit messages are in the proper format (see below)
- Push your changes to a topic branch in your fork of the repository
- Submit a pull request

Example:

``` shell
git remote add upstream https://github.com/glyn/nginx_robot_access.git
git checkout -b my-new-feature main
git commit -a
git push origin my-new-feature
```

### Conventions

Basic conventions around source file formatting are captured in the `.editorconfig` file.
Many editors support that file natively. Others require a plugin, see https://editorconfig.org/.

### Making changes

Be sure to run any tests:

``` shell
cargo test
```

### Staying in sync with upstream

When your branch gets out of sync with the glyn/nginx_robot_access/main branch, use the following to update:

``` shell
git checkout my-new-feature
git fetch -a
git pull --rebase upstream main
git push --force-with-lease origin my-new-feature
```

### Updating pull requests

If your PR fails to pass CI or needs changes based on code review, you'll most likely want to make some changes and push them to your branch.

Be sure to add a comment to the PR indicating your new changes are ready to review, as GitHub does not generate a
notification when you git push.

### Formatting commit messages

We follow the conventions on [How to Write a Git Commit Message](https://chris.beams.io/posts/git-commit/).

## Reporting bugs and creating issues

When opening a new issue, try to roughly follow the commit message format conventions above.
