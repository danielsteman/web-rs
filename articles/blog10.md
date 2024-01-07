% id: 10
% title: My default pre-commit hooks for Python projects 🪝🐍
% date: 2023-08-15
% tags: productivity

My goal is to automate as much as possible in my workflow, to decrease cognitive overhead and leave more headspace for actual work. Things that contribute to this goal are [shortkeys](https://github.com/danielsteman/.dotfiles/blob/master/nvim/lua/remap.lua), linters that are [triggered on save](https://code.visualstudio.com/docs/editor/codebasics), [pre-commit](https://pre-commit.com/) hooks and much more. I want to talk a little bit more about the last one: pre-commit hooks. As the name reveals, these hooks, or scripts, are ran before submission of new code. A script can be a check if the contributor is trying to [submit secrets](https://github.com/Yelp/detect-secrets), has [unused imports](https://beta.ruff.rs/docs/) or [wrong indentation](https://pypi.org/project/black/), just to name a view. Also, you can easily create your own hook and run a script that you wrote. Lately, I created a hook to generate a section of my CI/CD pipeline based on the folder-structure of my repository, but there are countless use cases, so be creative. The best thing about `pre-commit` is that its config (`.pre-commit-config.yaml`) is tracked by Git, so every contributor gets to use the same "commit rules". I'll quickly show a couple of the pre-commit hooks that I find very convenient in Python projects:

```yaml
repos:
  - repo: https://github.com/psf/black
    rev: 23.7.0
    hooks:
      - id: black
        language_version: python3.10
        args: [--line-length=88]

  - repo: https://github.com/astral-sh/ruff-pre-commit
    rev: v0.0.278
    hooks:
      - id: ruff

  - repo: https://github.com/pycqa/isort
    rev: 5.12.0
    hooks:
      - id: isort
        name: isort (python)
```

From top to bottom: `black` is a Python code formatter that adheres to the [PEP8](https://peps.python.org/pep-0008/) style guide and is opinioniated. With this small addition to your `settings.json` in VSCode, you can use `black` to format your code when you save the file:

```toml
"[python]": {
    "editor.defaultFormatter": "ms-python.black-formatter",
    "editor.formatOnSave": true
}
```

Since using this solution in VSCode, I find myself often writing code with sloppy whitespaces and indentation because I know it will get formatted on save. Very convenient and fast. Talking about fast, the second hook that I use a lot is `ruff`. This is basically `flake8` (a style guide enforcer) on steriods (rebuilt with Rust). `ruff` helps me to identify redundant, or missing code and safeguards a level of hygiene in the project. The last hook that I recently added to my default config is `isort`, which helps me to order my imports. I always have a hard time keeping my imports ordered and frankly, I just don't want to be bothered with this kind of overhead. `isort` orders imports alphabetically, into sections and by type. It make Python code much more readable and it allows you to be sloppy and not spend time on singing the ABC song in your head.

You might ask yourself: how do you ensure that contributors are installing pre-commit hooks properly and not work around your carefully constucted rules that are described in `pre-commit-config.yaml`? I usually run the checks in the CI pipeline as well. So unfixed code should result in a failing pipeline and lets the contributor know to `pre-commit install`, `pre-commit run --all` and try again. If you happen to host your project on Github, you can even use [templates](https://github.com/marketplace/actions/pre-commit) that handle the logic.
