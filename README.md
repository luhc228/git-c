# git-c

[![Crates.io][crates-badge]][crates-url]

[crates-badge]: https://img.shields.io/crates/v/gitc.svg
[crates-url]: https://crates.io/crates/gitc

Clone git repository with not only git url(e.g: `git@github.com:<group>/<project>.git`), but also http url(e.g: `https://github.com/<group>/<project>/tree/<branch>`). It's helpful to clone a repository of a special branch quickly.

## Usage

Supports http url: 

```sh
gitc https://github.com/<group>/<project>

# run the command: git clone git@github.com:<group>/<project>.git
```

Even with branch:

```sh
gitc https://github.com/<group>/<project>/tree/<branch>

# run the command: git clone git@github.com:<group>/<project>.git -b <branch>
```

Also supports ssh url:
```sh
gitc https://github.com/<group>/<project>.git

# run the command: git clone https://github.com/<group>/<project>.git

gitc git@github.com:<group>/<project>.git

# run the command: git clone git@github.com:<group>/<project>.git
```

## LICENSE

MIT
