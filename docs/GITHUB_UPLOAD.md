# Upload Omnigne to GitHub

## Using Git

1. Extract `omnigne-starter.zip`.
2. Open a terminal inside the extracted `omnigne` folder. `Cargo.toml`, `README.md` and `.github` should be directly inside this folder.
3. Create a new **empty** GitHub repository named `omnigne` under your account or organization. Leave automatic README, license and gitignore creation unchecked.
4. Replace `YOUR-ACCOUNT` below with the repository owner and run:

```bash
git init -b main
git add .
git commit -m "Initialize Omnigne workspace and paper scanner"
git remote add origin https://github.com/YOUR-ACCOUNT/omnigne.git
git push -u origin main
```

Authenticate with your own GitHub account when Git requests it. Repository credentials do not belong in any configuration file or this chat.

Both Cargo lockfiles should be committed. The archive includes `.github` and other dotfiles; using `git add .` preserves them. Build outputs, local secrets and generated scanner journals are ignored.

## Using GitHub Desktop

Add the extracted `omnigne` directory as a local repository, create it when prompted, commit the files, then choose **Publish repository** and select the intended owner and visibility.

## Using the GitHub website

Upload the extracted project files rather than the ZIP. Ensure `Cargo.toml` is at the repository root and include `.github`, `.gitignore`, `.gitattributes`, `.editorconfig` and `.dockerignore`. Some file pickers hide dotfiles; Git or GitHub Desktop is more reliable for a complete initial upload.

After upload, open the Actions tab to inspect the Rust, Soroban-build and EVM-contract jobs. A published repository is not a deployed trading service.

