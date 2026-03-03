{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:Nixos/nixpkgs/25.11";
  };

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      pkgsFor = forAllSystems (system: import nixpkgs { inherit system; });
    in
    {
      # Development shell for each supported system
      devShells = forAllSystems (system:
        let
          pkgs = pkgsFor.${system};
        in
        {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [

            ];
            nativeBuildInputs = with pkgs; [
              rustc
              rustfmt
              rust-analyzer
              cargo
            ];
          };
        });
    };
}
