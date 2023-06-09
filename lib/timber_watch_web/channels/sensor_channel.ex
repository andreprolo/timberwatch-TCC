defmodule TimberWatchWeb.SensorChannel do
  use TimberWatchWeb, :channel

  alias TimberWatch.SensorManager
  alias TimberWatch.ChannelWatcher

  @impl true
  def join(
        "room:monitoring",
        %{
          "id" => sensor_id,
          "type" => sensor_type,
          "name" => sensor_name
        },
        socket
      ) do
    :ok = ChannelWatcher.monitor(:sensor, self(), {__MODULE__, :leave, [sensor_id]})

    case SensorManager.check_sensor_id(sensor_id) do
      :ok ->
        SensorManager.join_sensor(%{
          id: sensor_id,
          type: sensor_type,
          name: sensor_name
        })

        {:ok, assign(socket, %{sensor_id: sensor_id})}

      :in_use ->
        # {:error, %{message: "This sensor_id is currently being used: #{sensor_id}"}}
        :error
    end
  end

  @impl true
  def handle_in("new_metric", %{"new_value" => value}, socket) do
    SensorManager.new_metric(socket.assigns.sensor_id, value)
    {:noreply, socket}
  end

  def leave(sensor_id) do
    SensorManager.disconnected_sensor(sensor_id)
  end
end
