{
  inputs = {
    garnix-lib.url = "github:garnix-io/garnix-lib";
    Rust.url = "github:garnix-io/rust-module";
  };

  nixConfig = {
    extra-substituters = [ "https://cache.garnix.io" ];
    extra-trusted-public-keys = [ "cache.garnix.io:CTFPyKSLcx5RMJKfLo5EEPUObbA78b0YQ2DTCJXqr9g=" ];
  };

  outputs = inputs: inputs.garnix-lib.lib.mkModules {
    modules = [
      inputs.Rust.garnixModules.default
    ];

    config = { pkgs, ... }: {
      rust = {
        rust-project = {
          buildDependencies = [  ];
          devTools = [ pkgs.prettierd pkgs.bash pkgs.zsh pkgs.fish pkgs.tmux ];
          runtimeDependencies = [  ];
          src = ./.;
          webServer = null;
        };
      };

      garnix.deployBranch = "master";
    };
  };
}
