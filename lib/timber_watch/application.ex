defmodule TimberWatch.Application do
  # See https://hexdocs.pm/elixir/Application.html
  # for more information on OTP Applications
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    children = [
      # Start the Telemetry supervisor
      TimberWatchWeb.Telemetry,
      # Start the Ecto repository
      TimberWatch.Repo,
      # Start the PubSub system
      {Phoenix.PubSub, name: TimberWatch.PubSub},
      # Start Finch
      {Finch, name: TimberWatch.Finch},
      # Start the Endpoint (http/https)
      TimberWatchWeb.Endpoint,
      # Channel Monitoring 
      {TimberWatch.ChannelWatcher, :sensor},
      # Sensor Manager
      TimberWatch.SensorManager
      # Start a worker by calling: TimberWatch.Worker.start_link(arg)
      # {TimberWatch.Worker, arg}
    ]

    # See https://hexdocs.pm/elixir/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_one, name: TimberWatch.Supervisor]
    Supervisor.start_link(children, opts)
  end

  # Tell Phoenix to update the endpoint configuration
  # whenever the application is updated.
  @impl true
  def config_change(changed, _new, removed) do
    TimberWatchWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end
