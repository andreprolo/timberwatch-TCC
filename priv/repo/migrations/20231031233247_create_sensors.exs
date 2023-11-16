defmodule TimberWatch.Repo.Migrations.CreateSensors do
  use Ecto.Migration

  import Timescale.Migration

  def up do
    create table(:sensors) do
      add :name, :string
      add :type, :string

      timestamps()
    end

    create table("sensors_realtime", primary_key: false) do
      add :timestamp, :naive_datetime_usec, null: false
      add :value, :float
      add(:sensor_id, references(:sensors), null: false)
    end

    create_hypertable(:sensors_realtime, :timestamp)
  end

  def down do
    drop table("sensors_realtime")
    drop table("sensors")
  end
end
