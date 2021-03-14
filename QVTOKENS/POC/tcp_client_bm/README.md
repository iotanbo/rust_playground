# TCP CLIENT FOR BENCHMARKING

This a TCP client for benchmarking similar to https://github.com/wg/wrk
but using CUSTOM (not HTTP) protocols.


## Backlog

On linux, there are 2 separate queues: 
  * for established TCP connections
    /proc/sys/net/core/somaxconn  (4096 default)
    sysctl -w net.core.somaxconn=myValue

  * for SYN-only received connections
    /proc/sys/net/ipv4/tcp_max_syn_backlog  (1024 default)


Other system tuning settings:
  /proc/sys/net/ipv4/tcp_synack_retries  (5 default)
  /proc/sys/net/ipv4/tcp_abort_on_overflow  (0 default)

## References

Tweaking system settings, TCP client issue on Ubuntu 16
https://github.com/tokio-rs/tokio/issues/383

How TCP backlog works in Linux:
http://veithen.io/2014/01/01/how-tcp-backlog-works-in-linux.html
