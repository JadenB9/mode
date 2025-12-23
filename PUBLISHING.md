# Publishing MODE to Package Managers

## Option 1: Publish to crates.io (Rust Package Registry)

This allows users to install with: `cargo install mode`

### Steps:

1. **Create a crates.io account**
   - Go to https://crates.io/
   - Sign in with GitHub

2. **Get your API token**
   ```bash
   cargo login
   ```
   - This will prompt you to paste your API token from https://crates.io/me

3. **Check if the name is available**
   ```bash
   cargo search mode
   ```
   - If "mode" is taken, consider: `mode-tui`, `mode-cli`, `shell-mode`, etc.
   - Update the name in `Cargo.toml` if needed

4. **Publish**
   ```bash
   cargo publish --dry-run  # Test first
   cargo publish            # Actually publish
   ```

5. **Users can now install with**
   ```bash
   cargo install mode
   ```

## Option 2: Create a Debian Package (.deb)

This allows users to install with: `sudo dpkg -i mode_0.1.0_amd64.deb`

### Using cargo-deb:

1. **Install cargo-deb**
   ```bash
   cargo install cargo-deb
   ```

2. **Add deb metadata to Cargo.toml**
   ```toml
   [package.metadata.deb]
   maintainer = "JadenB9 <jadenb9944@gmail.com>"
   copyright = "2025, JadenB9 <jadenb9944@gmail.com>"
   license-file = ["LICENSE", "4"]
   extended-description = """\
   A professional, blazing-fast terminal utility manager with a beautiful TUI interface. \
   Built with Rust and ratatui for smooth, stutter-free performance."""
   depends = "$auto"
   section = "utility"
   priority = "optional"
   assets = [
       ["target/release/mode", "usr/bin/", "755"],
       ["README.md", "usr/share/doc/mode/", "644"],
   ]
   ```

3. **Build the .deb package**
   ```bash
   cargo deb
   ```

4. **Distribute the .deb file**
   - Upload to GitHub Releases
   - Users can download and install:
     ```bash
     wget https://github.com/JadenB9/mode/releases/download/v0.1.0/mode_0.1.0_amd64.deb
     sudo dpkg -i mode_0.1.0_amd64.deb
     ```

## Option 3: Homebrew (macOS & Linux)

This allows users to install with: `brew install mode`

### Steps:

1. **Create a Homebrew tap repository**
   ```bash
   # On GitHub, create a new repo: homebrew-tap
   ```

2. **Create a formula** (in homebrew-tap repo)
   ```ruby
   # Formula/mode.rb
   class Mode < Formula
     desc "Blazing-fast terminal utility manager with TUI"
     homepage "https://github.com/JadenB9/mode"
     url "https://github.com/JadenB9/mode/archive/v0.1.0.tar.gz"
     sha256 "CALCULATE_THIS"
     license "MIT"

     depends_on "rust" => :build

     def install
       system "cargo", "install", *std_cargo_args
     end

     test do
       system "#{bin}/mode", "--version"
     end
   end
   ```

3. **Users install with**
   ```bash
   brew tap JadenB9/tap
   brew install mode
   ```

## Option 4: Snap Package (Ubuntu)

This allows users to install with: `sudo snap install mode`

1. **Create snapcraft.yaml**
2. **Publish to Snap Store**
3. Users: `sudo snap install mode`

## Option 5: AUR (Arch Linux)

This allows Arch users to install with: `yay -S mode` or `paru -S mode`

1. **Create a PKGBUILD file**
2. **Submit to AUR**
3. Arch users can install with their AUR helper

## Recommendation

**Start with:**
1. **crates.io** - Easiest, reaches all Rust users
2. **GitHub Releases with .deb** - Reaches Debian/Ubuntu users
3. **quick-install.sh** - Universal fallback

**Later add:**
4. Homebrew tap - For macOS users
5. AUR - For Arch users
6. Snap - For broader Ubuntu/Linux reach
