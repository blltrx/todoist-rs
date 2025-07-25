APP   <-----  SERVER GET TASKS


------


APP   <-- LOCAL CACHE <-- SERVER GET TASKS

- Cache sync attempts (in both directions) in separate threads
- App directly manipulates local cache
	- Cache contains db of tasks, along with queue of sync operations needed
	- Prioritises syncing outward, allows sync to fail if db no longer up to date
- Cache uses thread rx,tx to trigger new task updates when detected server side
- App uses thread rx,tx to gracefully close db on app closes

1. Redesign api.rs to meet new unified api spec for todoist -> move to cache module
2. Add a local cache
3. Add thread cache control

src/cache/mod.rs <- cache setup and control to interact with as a crate in src/app/mod.rs
src/cache/sync.rs <- sync and queue private functions/structs to govern background sync operations
src/cache/api.rs <- reworked post api to work with new unified todoist api

DO THIS IN A NEW BRANCH, protect working blocking version from conflicts in the mean time.
