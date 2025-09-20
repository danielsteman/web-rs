{
  description = "FastAPI project with dev shells, tests, linters, and pre-commit (macOS only)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, ... }:
    let
      system = "aarch64-darwin";
      pkgs = import nixpkgs { inherit system; };
    in {
      devShells.${system} = {
  
        # Local environment
        dev = pkgs.mkShell {
          buildInputs = [
            (pkgs.python311.withPackages (ps: with ps; [
              fastapi
              uvicorn
              httpx
              pydantic
              pytest      
            ]))
            pkgs.pre-commit
          ];

          shellHook = ''
            echo "FastAPI dev environment ready on macOS!"
            export ENV_FILE=.env
          '';
        };

        # Test environment
        test = pkgs.mkShell {
          buildInputs = [
            pkgs.python311
            pkgs.python311Packages.pip
            pkgs.python311Packages.setuptools
            pkgs.python311Packages.pytest
            pkgs.python311Packages.httpx
          ];
          shellHook = ''
            echo 'Test shell ready!
            pytest tests/
          '';
        };

        # Linting / code quality
        lint = pkgs.mkShell {
          buildInputs = [
            pkgs.python311
            pkgs.python311Packages.ruff
            pkgs.python311Packages.black
          ];
          shellHook = "echo 'Linting shell ready! Run flake8, black, isort'";
        };
      };

      packages.${system} = {
        pre-commit-hooks = pkgs.mkShell {
          buildInputs = [
            pkgs.python311
            pkgs.python311Packages.pip
            pkgs.pre-commit
          ];
          shellHook = ''
            echo "Run pre-commit hooks with: pre-commit run --all-files"
          '';
        };
      };
    };
}

