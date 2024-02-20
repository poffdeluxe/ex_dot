defmodule ExDotRustTest do
  use ExUnit.Case
  doctest ExDotRust

  test "greets the world" do
    assert ExDotRust.hello() == :world
  end
end
