defmodule DotTest do
  use ExUnit.Case
  doctest Dot

  test "greets the world" do
    assert Dot.hello() == :world
  end
end
