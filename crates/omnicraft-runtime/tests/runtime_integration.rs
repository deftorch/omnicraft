use omnicraft_runtime::{Context, App};
use tracing_subscriber::fmt::format::FmtSpan;
use std::cell::RefCell;
use std::rc::Rc;

fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .with_span_events(FmtSpan::CLOSE)
        .with_test_writer()
        .try_init();
}

#[test]
fn test_signal_effect_flow() {
    init_tracing();
    
    let mut ctx = Context::new();
    let count = ctx.create_signal(0);
    let output = Rc::new(RefCell::new(0));
    let output_clone = output.clone();
    let count_clone = count.clone(); // Signals are cheap and clonable

    // In the current simple implementation, we need to explicitly subscribe
    // because automatic dependency tracking isn't fully implemented yet.
    let output_clone_for_sub = output.clone();
    let count_clone_for_sub = count.clone();
    
    // Initial run
    {
        let val = count.get();
        tracing::info!("Initial run: value is {}", val);
        *output.borrow_mut() = val;
    }

    count.subscribe(move || {
        let val = count_clone_for_sub.get();
        tracing::info!("Subscriber run: value is {}", val);
        *output_clone_for_sub.borrow_mut() = val;
    });

    // Initial value
    assert_eq!(*output.borrow(), 0);

    // Update signal
    count.set(1);
    // Effects run immediately in the current simple implementation (or should)
    assert_eq!(*output.borrow(), 1);

    count.set(5);
    assert_eq!(*output.borrow(), 5);
}

#[test]
fn test_app_creation() {
    init_tracing();
    let app = App::new();
    assert_eq!(app.canvas_width(), 800.0);
}
