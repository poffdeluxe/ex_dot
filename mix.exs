defmodule ExDotRust.MixProject do
  use Mix.Project

  @source_url "https://github.com/poffdeluxe/ex_dot"
  @version "0.1.1-dev"
  @dev? String.ends_with?(@version, "-dev")
  @force_build? System.get_env("EX_DOT_BUILD") in ["1", "true"]

  def project do
    [
      app: :ex_dot,
      version: @version,
      elixir: "~> 1.15",
      start_permanent: Mix.env() == :prod,
      description: description(),
      deps: deps(),
      docs: docs(),
      package: package(),
      aliases: [
        "rust.lint": [
          "cmd cargo clippy --manifest-path=native/ex_dot/Cargo.toml -- -Dwarnings"
        ],
        "rust.fmt": [
          "cmd cargo fmt --manifest-path=native/ex_dot/Cargo.toml --all"
        ]
        # "localstack.setup": ["cmd ./test/support/setup-localstack.sh"],
        # ci: ["format", "rust.fmt", "rust.lint", "test"]
      ],
      source_url: @source_url
    ]
  end

  defp description() do
    "A Rustler-based wrapper around DOT-to-SVG functionality from the layout-rs crate."
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: []
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:ex_doc, ">= 0.0.0", only: :dev, runtime: false},
      {:rustler, "~> 0.31.0", optional: not (@dev? or @force_build?)},
      {:rustler_precompiled, "~> 0.7.1"}
    ]
  end

  defp docs do
    [
      main: "Dot",
      source_url: @source_url,
      source_ref: "v#{@version}",
      extras: ["demo.livemd"]
    ]
  end

  defp package do
    [
      name: "ex_dot",
      files: ~w(lib native checksum-*.exs mix.exs README.md LICENSE),
      licenses: ["Apache-2.0"],
      links: %{
        "GitHub" => @source_url
      },
      maintainers: ["Dr. Christian Geuer-Pollmann"]
    ]
  end
end
