defmodule TimberWatchWeb.MonitoringComponents do
  @moduledoc """
  Provides UI components for monitoring sensors.
  """
  use Phoenix.Component

  @doc """
  Renders a card with basic metrics.
  
  ## Examples
      <.metric_card value={50.0} img_src={~p"path/to/img"}/>
  """
  attr :value, :string, required: true
  attr :suffix_value, :string, required: true
  attr :img_src, :string, required: true
  attr :metric_name, :string, required: true
  attr :sensor_name, :string, required: true
  attr :color, :string, required: true

  def metric_card(assigns) do
    ~H"""
    <div class={"text-white w-64 bg-gradient-to-r p-4 py-5 px-5 rounded-xl" <> " #{@color}"}>
      <div class="grid grid-rows-2 grid-flow-col gap-2">
        <div>
          <h2><%= @metric_name %></h2>
          
          <p class="text-2xl font-bold"><%= @value %><%= @suffix_value %></p>
        </div>
        
        <div class="flex justify-between w-28 ">
          <div>
            <h3 class="text-xs">Nome</h3>
            
            <p class="font-bold"><%= @sensor_name %></p>
          </div>
        </div>
        
        <div class="row-span-2 flex items-center">
          <img class="w-20" src={@img_src} />
        </div>
      </div>
    </div>
    """
  end
end
