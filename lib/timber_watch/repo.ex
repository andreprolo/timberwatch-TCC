defmodule TimberWatch.Repo do
  use Ecto.Repo,
    otp_app: :timber_watch,
    adapter: Ecto.Adapters.Postgres
end
