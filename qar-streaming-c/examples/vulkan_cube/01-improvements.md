# Known gaps

Open work on the Vulkan cube example only. Runtime-side gaps live in the Quaternar registers.

- Windows only; Linux external-memory handles and console shutdown are not implemented.
- A launcher Stop is not graceful: the launcher closes the child's stdin and posts a non-forced
  `taskkill`, but the cube watches only its console handler and stop event, so the Stop escalates
  to a kill after ~1.3 s. Treat a closed stdin pipe as a stop (as `qar-executable-service` does).
- A failed target is retried after a fixed 3 s (`main.cpp`, `retries`). A kill-and-relaunch can take
  ~5 s to reconnect, so the cube waits a whole extra period. Retry on the sender's reconnect state,
  or back off from a short first delay.
