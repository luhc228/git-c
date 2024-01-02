# git-c

Clone repository with git url or http url.

## Usage

Supports http url: 

```sh
git-c https://github.com/<group>/<project>

Run: git clone https://github.com/<group>/<project>.git
```

Even with branch:

```sh
git-c https://github.com/<group>/<project>/tree/<branch>

# Run: git clone https://github.com/<group>/<project>.git -b <branch>
```

Also supports ssh url:
```sh
git-c https://github.com/<group>/<project>.git

# Run: git clone https://github.com/<group>/<project>.git

git-c git@github.com:<group>/<project>.git

# Run: git clone git@github.com:<group>/<project>.git
```
