defmodule TimberWatch.StocksGenerator do
  @ms_in_day :timer.hours(24)

  # import Timescale.Hyperfunctions
  # Repo.all(
  #   from(s in "stocks",
  #     where: s.symbol == "PETR4",
  #     select: {first(s.time, s.time), last(h.time, h.time)}
  #   )
  # )
  # Repo.all(from(s in "stocks", select: {time_bucket(s.time, "1 hour"), s.symbol, Timescale.Hyperfunctions.first(s.price, s.time), Timescale.Hyperfunctions.last(s.price, s.time)}, group_by: [time_bucket(s.time, "1 hour"), s.symbol]))
  
  def generate_heartbeats(stock, day) do
    do_generate_heartbeats(stock, [], NaiveDateTime.new!(day, ~T[00:00:00.000]), 0)
  end

  defp do_generate_heartbeats(stock, heartbeats, day, ms) do
    next = floor(:timer.minutes(1) / Enum.random(60..200)) + ms 

    if next < @ms_in_day do
      heartbeat = %{
        time: NaiveDateTime.add(day, next, :millisecond), 
        symbol: stock, 
        price: (Enum.random(1..10000)/100),
        day_volume: Enum.random(100..1000)
      } 
      do_generate_heartbeats(stock, [heartbeat | heartbeats], day, next)
    else
      Enum.reverse(heartbeats)
    end
  end
end

# batch_insert = fn heartbeats ->
#   heartbeats
#   |> Enum.chunk_every(100)
#   |> Enum.map(fn chunk ->
#     Repo.insert_all("stocks", chunk)
#   end)
# end
