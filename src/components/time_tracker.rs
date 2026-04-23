use leptos::prelude::*;

#[component]
pub fn TimeTracker() -> impl IntoView {
    // Stores the current timer state and elapsed time in seconds.
    let (is_running, set_is_running) = signal(false);
    let (seconds, set_seconds) = signal(0);

    // Advances the timer by one second while tracking is active.
    Effect::new(move |_| {
        if is_running.get() {
            let interval = std::time::Duration::from_secs(1);
            set_timeout(move || {
                set_seconds.update(|s| *s += 1);
            }, interval);
        }
    });

    // Formats elapsed time as HH:MM:SS for display.
    let formatted_time = move || {
        let s = seconds.get();
        let h = s / 3600;
        let m = (s % 3600) / 60;
        let sec = s % 60;
        format!("{:02}:{:02}:{:02}", h, m, sec)
    };

    view! {
        <div class="time-tracker-box">
            <div class="tracker-label">"Time Tracking"</div>
            <div class="tracker-main">
                <div class="timer-display" class:active=is_running>
                    {formatted_time}
                </div>
                
                <div class="tracker-controls">
                    {move || if !is_running.get() {
                        view! {
                            <button class="start-btn" on:click=move |_| set_is_running.set(true)>
                                "▶ Start"
                            </button>
                        }.into_any()
                    } else {
                        view! {
                            <button class="stop-btn" on:click=move |_| set_is_running.set(false)>
                                "■ Stop"
                            </button>
                        }.into_any()
                    }}
                </div>
            </div>
            
            // Static time log preview shown below the active tracker.
            <div class="time-logs">
                <div class="log-item">
                    <span>"Today, 14:20"</span>
                    <span class="log-duration">"1h 45m"</span>
                </div>
                <div class="log-item">
                    <span>"Yesterday, 09:15"</span>
                    <span class="log-duration">"3h 10m"</span>
                </div>
            </div>
        </div>
    }
}