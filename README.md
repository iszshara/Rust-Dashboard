To-Do

#1 add a sorter for processes block (maybe sort algorithm + the ui for it) > ✅ both Tasks completed!

#2 update the overall design (more color, etc.)     > ✅ Task completed!

#3 add a manual/help page for the usable keys when ESC is pressed + the option to quit  > ✅ Task completed!

#4 add options for color choosing       > Task canceled/delayed

#5 add option if user wants to see popup        > Task canceled/delayed

#6 add vertical bar chart for cpu core usage > ✅ Task completed!

#7 add uptime to dashboard     > ✅ Task completed!

#8 add fetch intervall, where the user can choose between 100ms and 1min (60000ms) > ✅ Task completed!

#9 add Error message if the window scaling is too small to display data correctly   > ✅ Task completed!

#13 improve the way the fetching works(
   1. Introduce Dependencies: add the tokio crate for asynchronous
       runtime and its multi-threaded capabilities.
   2. Create a Shared Data Store: define new struct to hold
      application's state (CPU data, process lists, etc.). This store will
      be wrapped in Arc<Mutex<...>> to allow safe, shared access between
      the data-fetching thread and the UI thread.
   3. Spawn a Background Fetching Task: A dedicated tokio task will be
      created. It will loop, periodically call sys.refresh_all(), and
      update the shared data store.
   4. Decouple Input Handling: User input will be handled in a separate
      task that sends events to the main UI loop through a channel,
      preventing any input lag.
   5. Adapt the Main UI Loop: The main loop will no longer fetch data
      directly. Instead, it will listen for events and, on every "tick," it
       will lock the shared data store to get the latest information for
      rendering.)


MAYBE

#10 add kill option for processes

#11 maybe add some symbols (e.g. to up and download)    > ✅ Task completed!

#12 maybe add scrollbar for processes block         > ✅ Task completed!

#13 maybe add option to choose between a second layout (or only one box at the time)
