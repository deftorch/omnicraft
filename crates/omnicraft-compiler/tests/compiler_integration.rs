use omnicraft_compiler::{compile_rust, CompilationTarget};
use tracing_subscriber::fmt::format::FmtSpan;

fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .with_span_events(FmtSpan::CLOSE)
        .with_test_writer()
        .try_init();
}

#[test]
fn test_compile_full_component() {
    init_tracing();

    let source = r##"
<script>
    const count = signal(0);
</script>

<canvas width={800} height={600}>
    <circle x={100} y={100} radius={50} fill="#FF0000" />
    <text content="Hello OmniCraft" x={200} y={200} />
</canvas>
"##;

    let result = compile_rust(source, "TestComponent");
    assert!(result.is_ok());

    let rust_code = result.unwrap();
    println!("{}", rust_code);

    // Verify key parts of generated code
    assert!(rust_code.contains("pub struct TestComponent"));
    assert!(rust_code.contains("pub count: Signal<f64>")); // inferred type
    assert!(rust_code.contains("Shape::Circle"));
    assert!(rust_code.contains("TextContent"));
}

#[test]
fn test_compile_with_reactivity() {
    init_tracing();

    let source = r#"
<script>
    let x = signal(10);
    let y = memo(() => x() * 2);
    
    effect(() => {
        console.log(y());
    });
</script>

<canvas>
    <rect width={x()} height={y()} />
</canvas>
"#;

    let result = compile_rust(source, "ReactiveComponent");
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    
    assert!(rust_code.contains("pub x: Signal<f64>"));
    // Current simple type inference sees the arrow function and returns Fn(), 
    // and codegen treats all reactives as Signal wrappers for now.
    assert!(rust_code.contains("pub y: Signal<Fn()>"));
    // The codegen implementation might differ slightly on how it exposes memos, 
    // code inspection showed it iterates variables.
    
    assert!(rust_code.contains("world.spawn_empty()"));
    assert!(rust_code.contains("Shape::Rectangle"));
}
