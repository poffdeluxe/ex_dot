defmodule Dot.Native do
  @moduledoc false

  mix_config = Mix.Project.config()
  version = mix_config[:version]
  github_url = mix_config[:package][:links]["GitHub"]
  mode = if Mix.env() in [:dev, :test], do: :debug, else: :release

  # use Rustler, otp_app: :ex_dot, crate: "ex_dot"
  use RustlerPrecompiled,
    otp_app: :ex_dot,
    crate: "ex_dot",
    base_url: "#{github_url}/releases/download/v#{version}",
    version: version,
    targets: ~w(
      x86_64-pc-windows-msvc
      x86_64-pc-windows-gnu
      x86_64-unknown-linux-gnu
      aarch64-unknown-linux-gnu
      x86_64-apple-darwin
      aarch64-apple-darwin
    ),
    nif_versions: ["2.15"],
    mode: mode,
    force_build: System.get_env("EX_DOT_BUILD") in ["1", "true"]

  def nif_dot_to_svg(_dot_string), do: :erlang.nif_error(:nif_not_loaded)
end
