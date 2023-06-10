defmodule TimberWatch.SensorManager do
  use GenServer

  use Phoenix.VerifiedRoutes,
    endpoint: TimberWatchWeb.Endpoint,
    router: TimberWatchWeb.Router,
    statics: TimberWatchWeb.static_paths()

  alias Phoenix.PubSub

  ## Client API

  def get_sensors() do
    GenServer.call(:sensor_manager, :get_sensors)
  end

  def check_sensor_id(sensor_id) do
    GenServer.call(:sensor_manager, {:check_sensor_id, sensor_id})
  end

  def join_sensor(sensor) do
    GenServer.cast(:sensor_manager, {:join_sensor, sensor})
  end

  def new_metric(sensor_id, value) do
    GenServer.cast(:sensor_manager, {:new_metric, sensor_id, value})
  end

  def disconnected_sensor(sensor_id) do
    GenServer.cast(:sensor_manager, {:disconnected_sensor, sensor_id})
  end

  ## Server API

  def start_link(_) do
    GenServer.start_link(__MODULE__, [], name: :sensor_manager)
  end

  def init(_) do
    {:ok, %{sensors: []}}
  end

  def handle_call(:get_sensors, _from, state) do
    {:reply, state.sensors, state}
  end

  def handle_call({:check_sensor_id, sensor_id}, _from, state) do
    status =
      cond do
        is_new_sensor?(%{id: sensor_id}, state) -> :ok
        Keyword.get(state.sensors, String.to_atom(sensor_id)).disconnected -> :ok
        true -> :in_use
      end

    {:reply, status, state}
  end

  def handle_cast({:join_sensor, params}, state) do
    new_sensors =
      if is_new_sensor?(params, state) do
        sensor = mount_new_sensor(params)
        broadcast_sensor_joined(sensor)

        Keyword.put(
          state.sensors,
          String.to_atom(params.id),
          sensor
        )
      else
        sensor =
          Keyword.get(state.sensors, String.to_atom(params.id))
          |> Map.put(:disconnected, false)
          |> Map.put(:last_activity, NaiveDateTime.utc_now())

        Keyword.put(
          state.sensors,
          String.to_atom(params.id),
          sensor
        )
      end

    {:noreply, Map.put(state, :sensors, new_sensors)}
  end

  def handle_cast({:new_metric, sensor_id, value}, state) do
    if not is_new_sensor?(%{id: sensor_id}, state) do
      value = adjust_sensor_value(value)
      broadcast_new_metric(%{sensor_id: sensor_id, value: value})

      updated_sensor =
        Keyword.get(state.sensors, String.to_atom(sensor_id))
        |> Map.put(:disconnected, false)
        |> Map.put(:value, value)
        |> Map.put(:last_activity, NaiveDateTime.utc_now())

      new_sensors = Keyword.put(state.sensors, String.to_atom(sensor_id), updated_sensor)
      {:noreply, Map.put(state, :sensors, new_sensors)}
    else
      {:noreply, state}
    end
  end

  def handle_cast({:disconnected_sensor, sensor_id}, state) do
    if not is_new_sensor?(%{id: sensor_id}, state) do
      broadcast_sensor_disconnected(sensor_id)

      updated_sensor =
        Keyword.get(state.sensors, String.to_atom(sensor_id))
        |> Map.put(:disconnected, true)
        |> Map.put(:last_activity, NaiveDateTime.utc_now())

      new_sensors = Keyword.put(state.sensors, String.to_atom(sensor_id), updated_sensor)
      {:noreply, Map.put(state, :sensors, new_sensors)}
    else
      {:noreply, state}
    end
  end

  defp mount_new_sensor(params) do
    %{
      id: params.id,
      type: params.type,
      name: params.name,
      color: generate_color(),
      icon: get_sensor_icon(params.type),
      disconnected: false,
      value: nil,
      suffix_value: get_suffix_value(params.type),
      last_activity: NaiveDateTime.utc_now()
    }
  end

  defp get_suffix_value(sensor_type) do
    case sensor_type do
      "temperature" -> "°C"
      "energy" -> "V"
      "sound" -> "dB"
      _ -> ""
    end
  end

  defp generate_color() do
    [
      "blue",
      "orange",
      "green",
      "yellow",
      "violet",
      "red",
      "cyan",
      "fuchsia",
      "rose",
      "indigo",
      "sky",
      "teal",
      "emerald",
      "lime",
      "amber"
    ]
    |> Enum.random()
  end

  defp get_sensor_icon(sensor_type) do
    case sensor_type do
      "temperature" -> ~p"/images/thermometer.png"
      "vibration" -> ~p"/images/vibration.png"
      "energy" -> ~p"/images/energy.png"
      "sound" -> ~p"/images/sound.png"
      _ -> ~p"/images/default.png"
    end
  end

  defp adjust_sensor_value(value) when is_float(value) do
    value
    |> Decimal.from_float()
    |> Decimal.round(2)
    |> Decimal.to_float()
  end

  defp adjust_sensor_value(value), do: value

  defp broadcast_sensor_joined(sensor) do
    PubSub.broadcast!(
      TimberWatch.PubSub,
      "metrics_watcher",
      {:sensor_joined, sensor}
    )
  end

  defp broadcast_new_metric(data) do
    PubSub.broadcast!(
      TimberWatch.PubSub,
      "metrics_watcher",
      {:new_metric, data}
    )
  end

  defp broadcast_sensor_disconnected(sensor_id) do
    PubSub.broadcast!(
      TimberWatch.PubSub,
      "metrics_watcher",
      {:sensor_disconnected, sensor_id}
    )
  end

  defp is_new_sensor?(params, state) do
    not Keyword.has_key?(state.sensors, String.to_atom(params.id))
  end
end
