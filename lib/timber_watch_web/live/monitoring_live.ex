defmodule TimberWatchWeb.MonitoringLive do
  use TimberWatchWeb, :live_view

  alias Phoenix.PubSub

  def render(assigns) do
    ~H"""
    <div class="flex items-center mb-5">
      <img class="w-12 mr-2" src={~p"/images/monitoring.png"} />
      <h1 class="text-3xl font-bold">Monitoring Page</h1>
    </div>

    <div class="grid grid-cols-3 gap-4">
      <.metric_card
        metric_name="Temperatura"
        sensor_name="Plaina 1"
        value={@temperature}
        color="from-blue-500 to-blue-200"
        img_src={~p"/images/thermometer.png"}
      />

      <.metric_card
        metric_name="Temperatura"
        sensor_name="Plaina 2"
        value={@temperature}
        color="from-green-500 to-green-200"
        img_src={~p"/images/thermometer.png"}
      />

      <.metric_card
        metric_name="Temperatura"
        sensor_name="Plaina 2"
        value={@temperature}
        color="from-orange-500 to-orange-200"
        img_src={~p"/images/thermometer.png"}
      />

      <.metric_card
        metric_name="Temperatura"
        sensor_name="Plaina 2"
        value={@temperature}
        color="from-red-500 to-red-200"
        img_src={~p"/images/thermometer.png"}
      />

      <.metric_card
        metric_name="Temperatura"
        sensor_name="Plaina 2"
        value={@temperature}
        color="from-yellow-500 to-yellow-200"
        img_src={~p"/images/thermometer.png"}
      />

      <.metric_card
        metric_name="Temperatura"
        sensor_name="Plaina 2"
        value={@temperature}
        color="from-violet-500 to-violet-200"
        img_src={~p"/images/thermometer.png"}
      />
    </div>
    """
  end

  def mount(_params, _session, socket) do
    PubSub.subscribe(TimberWatch.PubSub, "metrics_watcher")

    {:ok, assign(socket, :temperature, 0.0)}
  end

  def handle_info({:new_metric, data}, socket) do
    temperature =
      data.temperature
      |> String.to_float()
      |> Decimal.from_float()
      |> Decimal.round(2)
      |> Decimal.to_float()

    {:noreply, assign(socket, :temperature, temperature)}
  end
end
