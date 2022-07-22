defmodule TreeSitterExample do
  use Rustler, otp_app: :tree_sitter_example, crate: "tree_sitter_example"

  # When your NIF is loaded, it will override this function.
  # def add(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
  def stuff(_json), do: :erlang.nif_error(:nif_not_loaded)
end
