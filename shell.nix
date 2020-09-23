let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs { };
  naersk = pkgs.callPackage sources.naersk { };
  scratch = naersk.buildPackage {
    root = ./.;
    doCheck = true;
  };
in pkgs.mkShell {
  buildInputs = [ ]
    ++ (with pkgs; [ cargo rustc rustfmt cargo-edit ]);
}
