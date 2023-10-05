workspace {

  model {
    user = person "Woodworking Professional"
      timberWatchSystem = softwareSystem "Timber Watch" {
        timberWatchApi = container "Timber Watch Api" "Elixir/Phoenix" "Elixir/Phoenix"
        singlePageApplication = container "Web Application" "Provides a real-time view of all the metrics using Phoenix Live View" "HTML/JavaScript" "Web Browser"
        timberWatchDb = container "Database" "Stores sensors metrics in time-series tables" "TimescaleDB/PostgreSQL" "Database"
      }

      sensorSystem = softwareSystem "IOT Devices" "Collects all sort of metrics every second" " External System"

      user -> timberWatchApi "Visits timberwatch.com" "HTTPS"
      user -> singlePageApplication "View real-time data collected by the sensors"
      timberWatchApi -> singlePageApplication "Delivers to the user web browser"
      timberWatchApi -> timberWatchDb "Reads from and writes to" "TCP/SQL"
      sensorSystem -> timberWatchApi "Publishes real time data to" "WebSocket"
  }

  views {
    systemContext timberWatchSystem {
      include *
        autolayout lr
    }

    container timberWatchSystem {
      include *
        autolayout lr
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
