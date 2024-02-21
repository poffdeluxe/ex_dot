defmodule Dot do
  @moduledoc """
  Converts a string in [DOT Language](https://graphviz.org/doc/info/lang.html) to an SVG string.
  """

  def to_svg(binary) when is_binary(binary) do
    Dot.Native.nif_dot_to_svg(binary)
  end
end
