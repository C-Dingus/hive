{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  # nativeBuildInputs is typically used for tools needed to run or build the project.
  nativeBuildInputs = with pkgs; [
    rustc
    cargo
    SDL2
  ];

  # Optionally, you can define environment variables or run commands when entering the shell.
  shellHook = ''
    echo "added packages:"
    echo "rustc"
    echo "cargo"
    echo "SDL2"
  '';
}
