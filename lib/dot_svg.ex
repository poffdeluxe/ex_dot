defmodule DotSvg do
  @moduledoc """
  Converts a string in [DOT Language](https://graphviz.org/doc/info/lang.html) to an SVG string.
  """

  def dot_to_svg(binary) when is_binary(binary) do
    DotSvg.Native.nif_dot_to_svg(binary)
  end
end
