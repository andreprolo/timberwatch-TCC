require 'perlin_noise'
require 'websocket-client-simple'
require 'pry'

@ws = WebSocket::Client::Simple.connect 'ws://localhost:4000/socket/websocket?vsn=2.0.0' do |ws|
  ws.on :open do
    ws.send('["3","3","monitoring","phx_join",{}]')
  end

  ws.on :message do |msg|
    puts msg.data
  end
end

@noise = Perlin::Noise.new 1
@count = 0

def main()
  while true do
    @ws.on :message do |msg|
      puts msg.data
    end

    sleep(1)
    temp = get_temperature()

    puts temp
    @ws.send('["3","5","monitoring","new_temperature",{"temperature":'+temp.to_s+"}]")
  end 
end

def get_temperature()
    result = @noise[@count]
    @count += 0.01
    result
end

main()
