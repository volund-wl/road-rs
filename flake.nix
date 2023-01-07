{
  description = "Road-rs dev shell";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
		#rust-overlay = {
    #  url = "github:oxalica/rust-overlay";
    #  inputs.nixpkgs.follows = "nixpkgs";
    #};
    nci.url = "github:yusdacra/nix-cargo-integration";
    nci.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs:
    inputs.nci.lib.makeOutputs {
      root = ./.;
      config = common: {
        shell = {
					env = [
						{
							name = "PATH";
							eval = "$HOME/.cargo/bin:$PATH";
						}
						{
							name = "RUST_BACKTRACE";
							value = "1";
						}
					];
          packages = with common.pkgs; [ 
						rust-analyzer
						cbfmt
						treefmt
					];
        };
      };
    };
}
