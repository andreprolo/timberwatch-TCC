workspace {

  model {
    user = person "Woodworking Professional"
      timberWatchSystem = softwareSystem "TimberWatch" {
        timberWatchApi = container "TimberWatch Api" "Elixir/Phoenix" "Elixir/Phoenix" {
          timberWatchLiveView = component "Monitoring LiveView" "Manage connections to client web browsers and dinamyc changes the HTML in real time" "Phoenix LiveView Page"
          timberWatchPubSub = component "PubSub" "Realtime Publisher/Subscriber service" "Phoenix PubSub"
          timberWatchSensorManager = component "SensorManager" "In-memmory store of current conected sensors" "GenServer"
          sensorChannel = component "SensorChannel" "Manage connections with each sensor via websocket" "Phoenix Channel"
        }
        singlePageApplication = container "Web Browser" "Provides a real-time view of all the metrics using Phoenix Live View" "HTML/JavaScript" "Web Browser"
        //timberWatchDb = container "Database" "Stores sensors metrics in time-series tables" "TimescaleDB/PostgreSQL" "Database"
      }

      sensorSystem = softwareSystem "IOT Devices" "Collects all sort of metrics every second" "External System"

      user -> timberWatchApi "Visits timberwatch.com" "HTTPS"
      user -> singlePageApplication "View real-time data collected by the sensors"
      timberWatchApi -> singlePageApplication "Delivers to the user web browser"
      //timberWatchApi -> timberWatchDb "Reads from and writes to" "TCP/SQL"
      sensorSystem -> timberWatchApi "Publishes real time data to" "WebSocket"

      singlePageApplication -> timberWatchLiveView "Loads web page with sensors data" "HTTPS"
      timberWatchLiveView -> timberWatchPubSub "Subscribes to topic 'metrics_watcher'" "Pub/Sub"
      timberWatchLiveView -> timberWatchSensorManager "Fetch sensors data with method get_sensors()" "GenServer/Call"
      timberWatchSensorManager -> timberWatchPubSub "Notify updates made on sensors data" "Pub/Sub"
      timberWatchPubSub -> timberWatchLiveView "Broadcast updates made on sensors data" "Pub/Sub"
      sensorSystem -> sensorChannel "Sends real time data to" "WebSocket"
      sensorChannel -> timberWatchSensorManager "Register or updates a sensor" "GenServer/Cast"
      //timberWatchSensorManager -> timberWatchDb "Save sensor state" "TCP/SQL"
  }

  views {
    systemContext timberWatchSystem {
      include *
        autolayout lr
    }

    container timberWatchSystem {
      include *
        //autolayout lr
    }

    component timberWatchApi {
      include * user
      //autolayout 
    }

    styles {
      element "Web Browser" {
        shape WebBrowser
      }
      element "Mobile App" {
        shape MobileDeviceLandscape
      }
      element "Database" {
        shape Cylinder
      }
      element "External System" {
        background #999999
          color #ffffff
      }
    }

    theme default
  }
}
