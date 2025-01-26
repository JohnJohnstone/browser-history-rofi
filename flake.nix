{
  description = "browser-history devShell and package";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustbin = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default);
        myrustPlatform = pkgs.makeRustPlatform {
            cargo = rustbin;
            rustc = rustbin;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            just
            openssl
            sqlite

            pkg-config

            eza
            fd
            glib
            cairo
            pango
            rustbin
            clippy
            rust-analyzer
            wine64
            pkgsCross.mingwW64.stdenv.cc
          ];

          shellHook = ''
            alias ls=exa
            alias find=fd
          '';
        };

        packages.browserhistory-rofi = 
            myrustPlatform.buildRustPackage {
                name = "browser-history-rofi";
                src = ./.;
                PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig:${pkgs.glib.dev}/lib/pkgconfig:${pkgs.cairo.dev}/lib/pkgconfig:${pkgs.pango.dev}/lib/pkgconfig:${pkgs.pango.dev}/lib:${pkgs.pango.dev}/include/pango-1.0";

                RUSTFLAGS="--cfg rofi_next";
                nativeBuildInputs = with pkgs; [
                    pkg-config 
                    glib
                    cairo
                    pango
                    # openssl
                    # openssl.dev
                    rofi-wayland
                ];
                buildInputs = [
                    # libxkbcommon
                    pango
                    cairo
                    rofi-wayland
                ];
                cargoLock = {
                    lockFile = ./Cargo.lock;
                    outputHashes = {
                        "urlhandler-0.1.0" = "8psgRIV+tcrXTON2LoqMBUX/LSF3eA+kdtz2/C9lVQg=";
                    };
                };
                buildPhase = ''
cargo build --release
# ls -lah target/release
'';
                installPhase = ''
                    mkdir -p $out/lib/rofi/
                    # echo "ls out"
                    # ls -lah $out
                    # echo "end out"
                    # echo "curr dir"
                    # echo $(pwd)
                    # echo "ls -lah"
                    # ls -lah
                    # echo "ls target/release/"
                    # ls -lah target/release/
                    cp target/release/libbrowserhistory_rofi.so $out/lib/rofi/libbrowserhistory_rofi.so
                    '';
                    };
             }
    );
}
