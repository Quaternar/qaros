# Known gaps

Open work on the Vulkan cube example only. Runtime-side gaps live in the Quaternar registers.

- Windows only; Linux external-memory handles and console shutdown are not implemented.
- A launcher Stop closes the cube's stdin pipe, which the cube treats as a stop. Its teardown time
  (senders, session, runtime) is not measured against the launcher's 1 s grace period; a longer
  teardown is still ended by a kill.
- A failed target is retried after a fixed 3 s (`main.cpp`, `retries`). A kill-and-relaunch can take
  ~5 s to reconnect, so the cube waits a whole extra period. Retry on the sender's reconnect state,
  or back off from a short first delay.
- The executable is unsigned, so QAROS approves it by path and hash: every rebuild asks for approval
  again. A signed build would be admitted across updates.
