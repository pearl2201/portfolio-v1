defmodule PortfolioApi.Repo do
  use Ecto.Repo,
    otp_app: :portfolio_api,
    adapter: Ecto.Adapters.Postgres
end
