defmodule TimberWatch.UseCases.Sensor do
  
  import Ecto.Query, warn: false
  alias TimberWatch.Repo
  alias TimberWatch.Schemas.Sensor

  def create_or_update_sensor(attrs \\ %{}) do
    if Map.has_key?(attrs, :id) do
      already_exists? = Repo.all(from s in "sensors", select: s.id, where: s.id == ^attrs.id) |> Enum.empty? |> Kernel.not
       
      if already_exists? do
        %Sensor{}
        |> Sensor.changeset(attrs)
        |> Repo.update()
      else
        %Sensor{}
        |> Sensor.changeset(attrs)
        |> Repo.insert()
      end
    else
      %Sensor{}
      |> Sensor.changeset(attrs)
      |> Repo.insert()
    end
  end
end
