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
      <%= if Enum.empty?(@sensors) do %>
        <h2 class="text-gray-400">-- No sensors connected --</h2>
      <% end %>

      <%= for sensor <- Keyword.values(Enum.sort(@sensors)) do %>
        <.metric_card
          metric_name={metric_name(sensor.type)}
          sensor_name={sensor.name}
          disconnected={sensor.disconnected}
          value={sensor.value}
          suffix_value={sensor.suffix_value}
          color={pick_color(sensor)}
          img_src={sensor.icon}
        />
      <% end %>
    </div>
    """
  end

  def mount(_params, _session, socket) do
    PubSub.subscribe(TimberWatch.PubSub, "metrics_watcher")
    sensors = TimberWatch.SensorManager.get_sensors()
    {:ok, assign(socket, :sensors, sensors)}
  end

  def handle_info({:sensor_joined, sensor}, socket) do
    new_sensors =
      Keyword.put(
        socket.assigns.sensors,
        String.to_atom(sensor.id),
        sensor
      )

    {:noreply, assign(socket, :sensors, new_sensors)}
  end

  def handle_info({:new_metric, params}, socket) do
    sensor = Keyword.get(socket.assigns.sensors, String.to_atom(params.sensor_id))

    new_sensors =
      if Kernel.is_nil(sensor) do
        socket.assigns.sensors
      else
        updated_sensor =
          sensor
          |> Map.put(:value, params.value)
          |> Map.put(:disconnected, false)

        Keyword.put(
          socket.assigns.sensors,
          String.to_atom(params.sensor_id),
          updated_sensor
        )
      end

    {:noreply, assign(socket, :sensors, new_sensors)}
  end

  def handle_info({:sensor_disconnected, sensor_id}, socket) do
    sensor =
      Keyword.get(socket.assigns.sensors, String.to_atom(sensor_id))
      |> Map.put(:disconnected, true)

    new_sensors = Keyword.put(socket.assigns.sensors, String.to_atom(sensor_id), sensor)
    {:noreply, assign(socket, :sensors, new_sensors)}
  end

  defp pick_color(sensor) do
    color =
      if sensor.disconnected do
        "gray"
      else
        sensor.color
      end

    case color do
      "blue" -> "from-blue-500 to-blue-200"
      "orange" -> "from-orange-500 to-orange-200"
      "green" -> "from-green-500 to-green-200"
      "yellow" -> "from-yellow-500 to-yellow-200"
      "violet" -> "from-violet-500 to-violet-200"
      "red" -> "from-red-500 to-red-200"
      "cyan" -> "from-cyan-500 to-cyan-200"
      "fuchsia" -> "from-fuchsia-500 to-fuchsia-200"
      "rose" -> "from-rose-500 to-rose-200"
      "indigo" -> "from-indigo-500 to-indigo-200"
      "sky" -> "from-sky-500 to-sky-200"
      "teal" -> "from-teal-500 to-teal-200"
      "emerald" -> "from-emerald-500 to-emerald-200"
      "lime" -> "from-lime-500 to-lime-200"
      "amber" -> "from-amber-500 to-amber-200"
      _ -> "from-gray-500 to-gray-200"
    end
  end

  defp metric_name(sensor_type) do
    case sensor_type do
      "temperature" -> "Temperatura"
      "vibration" -> "Vibração"
      "energy" -> "Energia"
      "sound" -> "Ruído"
      _ -> "Genérico"
    end
  end
end
