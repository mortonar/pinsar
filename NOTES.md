* Use `sadf` to parse `/var/log/sa/saXX` files since parsing the human-readable versions is just UGLY.


* Flow: Source (System) → Data Acquisition → Data Model → Data Processing → Visualization (Graph)
  * Data Acquisition: sar_collector -- run `sadf` on local or remote host, parse output
    * Provide functions for different data collection needs, e.g., collect_cpu_stats(), collect_network_stats(), collect_from_file(path).
  * Data Model: sar_model -- define data structures for sar data
    * serde-based
  * Data Processing: sar_processor -- functions to filter, aggregate, and transform data
    * e.g., filter_by_time_range(data, start_time, end_time), aggregate_by_interval(data, interval)
  * Visualization: sar_visualizer -- use a plotting library to create graphs

---

Collect sar data in JSON with:
```shell
mkdir resources
ssh user@host "sadf -j /var/log/sa/sa## -- -A" > resources/vm1.sar.json
```

Useful jq commands:
* List all metric types in a sar JSON file:
```shell
jq -C '.sysstat.hosts[0].statistics[1] | keys' resources/vm1.sar.json
[
  "cpu-load",
  "disk",
  "hugepages",
  "io",
  "kernel",
  "memory",
  "network",
  "paging",
  "process-and-context-switch",
  "queue",
  "swap-pages",
  "timestamp"
]
```

* See [Time Series Chart](https://plotters-rs.github.io/book/basic/basic_data_plotting.html) example in plotters docs.
  * Basic plotters usage is to create a `BitMapBackend`, then create a `ChartBuilder` on top of that, then configure the chart with axes and labels, then draw series on the chart.
  * chrono's `DateTime` [can be used](https://docs.rs/plotters/latest/plotters/coord/ranged1d/trait.AsRangedCoord.html#impl-AsRangedCoord-for-Range%3CDateTime%3CZ%3E%3E) for time-based x-axis.
  * `sadf` outputs timestamps in UTC by default. Perhaps we can convert this to local for now and better handle later?