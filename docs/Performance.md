**inlining**! Ini adalah teknik optimasi kompilator yang sangat powerful dan relevan dengan desain OmniCraft. Mari saya jelaskan secara mendalam.

---

## 🎯 Apa Itu Inlining?

**Inlining** adalah proses di mana kompilator menggantikan pemanggilan fungsi dengan body fungsi itu sendiri secara langsung. Bayangkan seperti "copy-paste" kode fungsi ke tempat di mana fungsi tersebut dipanggil, yang dilakukan otomatis oleh kompilator pada saat kompilasi.

### Ilustrasi Sederhana

Katakanlah Anda memiliki kode seperti ini:

```rust
// Sebelum inlining
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn calculate() -> i32 {
    let x = add(5, 3);  // Function call
    let y = add(x, 2);  // Function call
    y
}
```

Setelah kompilator melakukan inlining, kode yang sebenarnya dieksekusi menjadi seperti ini:

```rust
// Setelah inlining (yang dilakukan kompilator)
fn calculate() -> i32 {
    let x = 5 + 3;  // Langsung dikerjakan, tidak ada function call!
    let y = x + 2;  // Langsung dikerjakan
    y
}
```

Perhatikan bahwa function call `add()` telah hilang dan digantikan dengan operasi langsung. Ini menghilangkan overhead dari pemanggilan fungsi.

---

## 🔍 Mengapa Inlining Penting?

### 1. **Menghilangkan Overhead Function Call**

Setiap kali Anda memanggil fungsi, ada biaya tersembunyi:

```
Function call overhead:
1. Push parameter ke stack      (~2-5 CPU cycles)
2. Jump ke alamat fungsi        (~1-2 CPU cycles)
3. Setup stack frame baru       (~3-5 CPU cycles)
4. Execute function body        (varies)
5. Cleanup stack frame          (~2-3 CPU cycles)
6. Return ke caller             (~1-2 CPU cycles)
7. Pop parameter dari stack     (~2-5 CPU cycles)

Total overhead: ~11-24 CPU cycles
```

Untuk fungsi yang sangat kecil seperti `add(a, b)`, overhead ini bisa lebih besar daripada pekerjaan fungsi itu sendiri! Dengan inlining, semua overhead ini hilang.

### 2. **Memungkinkan Optimasi Lebih Lanjut**

Ini adalah benefit tersembunyi yang sangat powerful. Ketika kompilator melihat kode setelah inlining, kompilator bisa melakukan optimasi tambahan yang sebelumnya tidak mungkin.

Contoh:

```rust
// Sebelum inlining
fn square(x: i32) -> i32 {
    x * x
}

fn calculate_area() -> i32 {
    let width = 5;
    square(width)  // Compiler tidak tahu nilai width saat compile time
}

// Setelah inlining
fn calculate_area() -> i32 {
    let width = 5;
    width * width  // Compiler bisa melihat width = 5!
}

// Setelah constant folding (optimasi tambahan)
fn calculate_area() -> i32 {
    25  // Dihitung pada compile time!
}
```

Tanpa inlining, kompilator tidak akan bisa melakukan constant folding karena tidak bisa "melihat ke dalam" fungsi `square()`.

---

## 🎨 Inlining dalam Konteks OmniCraft

Inlining sangat relevan untuk OmniCraft karena kita mengkompilasi dari DSL tingkat tinggi ke kode performa tinggi. Mari saya tunjukkan bagaimana ini bekerja:

### Contoh 1: Component Inlining

Katakanlah Anda punya komponen sederhana:

```omni
<!-- Button.omni -->
<component name="Button">
  <props>
    label: string
    x: number
    y: number
  </props>
  
  <canvas>
    <rect x={props.x} y={props.y} width={100} height={40} fill="#007bff" />
    <text x={props.x + 50} y={props.y + 20} content={props.label} />
  </canvas>
</component>
```

Dan Anda menggunakannya seperti ini:

```omni
<!-- App.omni -->
<canvas>
  <Button label="Click me" x={100} y={200} />
  <Button label="Submit" x={220} y={200} />
</canvas>
```

**Tanpa inlining**, kompilator akan menghasilkan kode seperti ini:

```rust
// Tidak optimal - ada function calls
fn render_button(label: &str, x: f32, y: f32, world: &mut World) {
    let rect = world.create_entity();
    world.add_component(rect, Transform { position: Vec2::new(x, y) });
    world.add_component(rect, Shape::Rectangle { width: 100.0, height: 40.0 });
    
    let text = world.create_entity();
    world.add_component(text, Transform { position: Vec2::new(x + 50.0, y + 20.0) });
    world.add_component(text, Text { content: label.to_string() });
}

fn render_app(world: &mut World) {
    render_button("Click me", 100.0, 200.0, world);  // Function call overhead
    render_button("Submit", 220.0, 200.0, world);    // Function call overhead
}
```

**Dengan inlining aggressive**, kompilator menghasilkan:

```rust
// Optimal - inlined dan constant-folded!
fn render_app(world: &mut World) {
    // Button 1 - fully inlined
    let rect1 = world.create_entity();
    world.add_component(rect1, Transform { position: Vec2::new(100.0, 200.0) });
    world.add_component(rect1, Shape::Rectangle { width: 100.0, height: 40.0 });
    
    let text1 = world.create_entity();
    world.add_component(text1, Transform { position: Vec2::new(150.0, 220.0) }); // 100+50, 200+20 computed at compile time!
    world.add_component(text1, Text { content: "Click me".to_string() });
    
    // Button 2 - fully inlined
    let rect2 = world.create_entity();
    world.add_component(rect2, Transform { position: Vec2::new(220.0, 200.0) });
    world.add_component(rect2, Shape::Rectangle { width: 100.0, height: 40.0 });
    
    let text2 = world.create_entity();
    world.add_component(text2, Transform { position: Vec2::new(270.0, 220.0) }); // 220+50, 200+20 computed!
    world.add_component(text2, Text { content: "Submit".to_string() });
}
```

Perhatikan beberapa hal penting:

1. **Tidak ada function calls** - semua di-inline langsung
2. **Arithmetic computed at compile time** - `x + 50` dan `y + 20` sudah dihitung menjadi `150.0` dan `220.0`
3. **Strings are compile-time constants** - `"Click me"` dan `"Submit"` tidak perlu computed at runtime

---

## 🚀 Levels of Inlining dalam OmniCraft

Dalam konteks kompilator OmniCraft, kita bisa menerapkan inlining pada berbagai level:

### Level 1: Expression Inlining

```omni
<script>
  const radius = signal(50);
  const diameter = signal(() => radius() * 2);
</script>

<canvas>
  <circle radius={diameter() / 2} />
</canvas>
```

Kompilator bisa melihat bahwa `diameter() / 2` sebenarnya adalah `(radius() * 2) / 2` yang bisa disederhanakan menjadi `radius()`:

```rust
// Setelah inlining dan algebraic simplification
<circle radius={radius()} />
```

### Level 2: Component Inlining (seperti contoh di atas)

Kompilator menggantikan penggunaan komponen dengan body-nya langsung.

### Level 3: Signal Accessor Inlining

```omni
<script>
  const count = signal(0);
</script>
```

Tanpa inlining, setiap `count()` adalah function call ke getter signal:

```rust
// Tidak optimal
fn get_count(&self) -> i32 {
    self.count.get()  // Function call
}

let value = self.get_count();  // Another function call
```

Dengan inlining:

```rust
// Optimal - direct memory access
let value = *self.count.value.borrow();  // Langsung akses memory!
```

### Level 4: Control Flow Inlining

```omni
{#if showCircle()}
  <circle x={400} y={300} radius={50} />
{/if}
```

Jika kompilator bisa membuktikan bahwa `showCircle()` selalu `true` pada compile time:

```rust
// Setelah dead code elimination via inlining
<circle x={400} y={300} radius={50} />
// Branch dihilangkan sepenuhnya!
```

---

## ⚖️ Trade-offs: Kapan TIDAK Melakukan Inlining?

Inlining bukan silver bullet. Ada situasi di mana inlining justru merugikan:

### 1. **Code Bloat**

Jika Anda inline fungsi besar yang dipanggil di banyak tempat, ukuran binary bisa membengkak:

```rust
// Fungsi besar (100 lines)
fn complex_calculation() {
    // ... 100 lines of code
}

// Dipanggil 50 kali
complex_calculation();  // Call #1
complex_calculation();  // Call #2
// ... 48 more calls

// Jika di-inline semuanya:
// Binary size = 100 lines × 50 calls = 5000 lines of duplicated code!
```

Ini buruk untuk:
- **Cache performance** - Code yang terlalu besar tidak muat di CPU instruction cache
- **Binary size** - File menjadi sangat besar
- **Compile time** - Kompilasi menjadi lambat

### 2. **Dynamic Dispatch Requirement**

Jika Anda butuh polymorphism atau late binding:

```rust
trait Drawable {
    fn draw(&self);
}

// Tidak bisa di-inline karena kita tidak tahu type konkrit
fn render_all(items: &[Box<dyn Drawable>]) {
    for item in items {
        item.draw();  // Dynamic dispatch - runtime decision
    }
}
```

### 3. **Recursive Functions**

```rust
fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)  // Rekursi
    }
}

// Tidak bisa di-inline sempurna karena recursive!
// Akan menyebabkan infinite expansion
```

---

## 🎯 Smart Inlining Strategy untuk OmniCraft

Berdasarkan pemahaman ini, inilah strategi inlining optimal untuk OmniCraft:

### Rule 1: Always Inline Small Functions

```rust
// Kompilator OmniCraft
impl InliningOptimizer {
    fn should_inline(&self, function: &Function) -> bool {
        // Inline jika function <= 10 instructions
        if function.instruction_count() <= 10 {
            return true;
        }
        
        // Inline jika hanya dipanggil sekali
        if function.call_count() == 1 {
            return true;
        }
        
        // Inline jika dalam hot path (dipanggil setiap frame)
        if function.is_in_hot_path() && function.instruction_count() <= 50 {
            return true;
        }
        
        false
    }
}
```

### Rule 2: Inline dengan Constant Folding

```rust
impl Optimizer {
    fn inline_with_constant_folding(&mut self, call: &FunctionCall) {
        // 1. Inline function
        let inlined_body = self.inline_function_body(call);
        
        // 2. Propagate constants
        let with_constants = self.propagate_constants(inlined_body);
        
        // 3. Fold constants
        let folded = self.fold_constants(with_constants);
        
        // 4. Eliminate dead code
        let final_code = self.eliminate_dead_code(folded);
        
        // Replace original call with optimized code
        self.replace_call(call, final_code);
    }
}
```

Contoh hasil:

```rust
// Original
let result = calculate_area(5);

// After inline_with_constant_folding
let result = 25;  // Semuanya dihitung compile time!
```

### Rule 3: Partial Inlining

Untuk fungsi besar, inline hanya bagian yang "hot" (sering dieksekusi):

```rust
fn complex_render(x: f32, y: f32) {
    // Hot path - inline this
    if simple_bounds_check(x, y) {
        quick_render(x, y);
    } else {
        // Cold path - don't inline, call normally
        expensive_fallback_render(x, y);
    }
}
```

---

## 💡 Kesimpulan untuk OmniCraft

Inlining adalah teknik kunci untuk mencapai performa yang Anda targetkan di OmniCraft. Dengan mengkombinasikan:

1. **Aggressive component inlining** - Komponen kecil di-inline sepenuhnya
2. **Expression inlining** - Computed values dievaluasi compile-time
3. **Signal accessor inlining** - Akses reaktif menjadi direct memory access
4. **Smart heuristics** - Tahu kapan inline dan kapan tidak

Anda bisa mencapai "zero-cost abstraction" yang dijanjikan oleh Rust, di mana kode tingkat tinggi yang indah dikompilasi menjadi kode machine-level yang sangat efisien, seolah-olah Anda menulis assembly langsung!

# 🚀 Advanced Compiler Optimizations untuk OmniCraft

Inlining hanyalah permulaan. Mari saya ajak Anda menjelajahi dunia optimasi kompilator tingkat profesional yang bisa diterapkan di OmniCraft. Saya akan jelaskan setiap teknik dari yang paling fundamental hingga yang paling advanced, lengkap dengan contoh konkrit bagaimana masing-masing bisa meningkatkan performa secara dramatis.

---

## 1. **Constant Folding & Constant Propagation** 📐

### Apa Itu?

**Constant Folding** adalah teknik di mana kompilator mengevaluasi ekspresi yang semua operandnya adalah konstanta pada waktu kompilasi, bukan runtime. **Constant Propagation** adalah teknik melacak nilai konstanta melalui program dan menggantikan penggunaan variabel dengan nilai konstanta tersebut.

Mari saya tunjukkan dengan contoh sederhana terlebih dahulu:

```javascript
// Kode yang Anda tulis
const x = 5;
const y = 10;
const z = x + y * 2;
const result = z + 15;
```

Tanpa optimasi, kompilator akan menghasilkan instruksi untuk setiap operasi:

```rust
// Compiled (tidak optimal)
let x = 5;
let y = 10;
let temp1 = y * 2;      // Runtime: multiplication
let temp2 = x + temp1;  // Runtime: addition
let z = temp2;
let result = z + 15;    // Runtime: addition
```

Dengan Constant Folding dan Propagation:

```rust
// Compiled (optimal)
let result = 40;  // Semua dihitung compile-time: 5 + (10 * 2) + 15 = 40
```

Perhatikan bahwa semua komputasi hilang sama sekali! Tidak ada operasi aritmatika yang perlu dilakukan saat runtime. Ini adalah bentuk optimasi paling powerful karena benar-benar menghilangkan pekerjaan.

### Implementasi dalam OmniCraft

Dalam konteks OmniCraft, ini sangat berguna untuk mengevaluasi layout calculations, transformations, dan color manipulations:

```omni
<script>
  const baseSize = 100;
  const scale = 1.5;
  const padding = 20;
  
  // Kompilator bisa menghitung ini semua compile-time
  const totalWidth = baseSize * scale + padding * 2;  // = 190
  const centerX = totalWidth / 2;  // = 95
</script>

<canvas>
  <rect 
    x={padding} 
    y={padding} 
    width={baseSize * scale}  
    height={baseSize * scale}
  />
</canvas>
```

Setelah constant folding:

```rust
// Generated code - semua pre-computed!
world.add_component(entity, Transform {
    position: Vec2::new(20.0, 20.0),  // padding
});
world.add_component(entity, Shape::Rectangle {
    width: 150.0,   // 100 * 1.5
    height: 150.0,  // 100 * 1.5
});
```

Implementasi dalam kompilator:

```rust
// compiler/src/optimizer/constant_folding.rs

pub struct ConstantFolder {
    constants: HashMap<String, Literal>,
}

impl ConstantFolder {
    pub fn fold_expression(&mut self, expr: &Expression) -> Expression {
        match expr {
            Expression::Binary { left, op, right } => {
                // Rekursif fold kedua operand dulu
                let left_folded = self.fold_expression(left);
                let right_folded = self.fold_expression(right);
                
                // Cek apakah keduanya konstanta
                if let (Expression::Literal(l), Expression::Literal(r)) = 
                    (&left_folded, &right_folded) 
                {
                    // Evaluasi pada compile time!
                    return self.evaluate_binary_op(l, op, r);
                }
                
                // Jika tidak bisa fold, kembalikan expression yang sudah di-fold sebagian
                Expression::Binary {
                    left: Box::new(left_folded),
                    op: *op,
                    right: Box::new(right_folded),
                }
            }
            
            Expression::Identifier(name) => {
                // Constant propagation - ganti variable dengan nilainya jika tahu
                if let Some(value) = self.constants.get(name) {
                    Expression::Literal(value.clone())
                } else {
                    expr.clone()
                }
            }
            
            _ => expr.clone()
        }
    }
    
    fn evaluate_binary_op(&self, left: &Literal, op: &BinaryOp, right: &Literal) 
        -> Expression 
    {
        match (left, op, right) {
            (Literal::Number(a), BinaryOp::Add, Literal::Number(b)) => {
                Expression::Literal(Literal::Number(a + b))
            }
            (Literal::Number(a), BinaryOp::Mul, Literal::Number(b)) => {
                Expression::Literal(Literal::Number(a * b))
            }
            (Literal::String(a), BinaryOp::Add, Literal::String(b)) => {
                Expression::Literal(Literal::String(format!("{}{}", a, b)))
            }
            // ... operasi lainnya
            _ => Expression::Binary {
                left: Box::new(Expression::Literal(left.clone())),
                op: *op,
                right: Box::new(Expression::Literal(right.clone())),
            }
        }
    }
}
```

---

## 2. **Dead Code Elimination (DCE)** 🗑️

### Apa Itu?

Dead Code Elimination adalah proses menghapus kode yang tidak akan pernah dieksekusi atau hasilnya tidak pernah digunakan. Ini mencakup berbagai bentuk:

**Unreachable Code** - Kode yang tidak akan pernah dijalankan:

```javascript
function example() {
    return 42;
    console.log("This never runs");  // Dead code!
}
```

**Unused Variables** - Variabel yang dideklarasikan tapi tidak pernah digunakan:

```javascript
const x = 10;  // Dead code jika x tidak pernah dipakai
const y = 20;
return y;
```

**Unused Functions** - Fungsi yang tidak pernah dipanggil:

```javascript
function unusedHelper() {  // Dead code
    return "never called";
}

function main() {
    return "result";
}
```

### Mengapa Penting?

Dead code tidak hanya membuat binary lebih besar, tapi juga membuang waktu kompilasi dan mempersulit maintenance. Dalam konteks WASM, setiap byte ekstra berarti waktu download dan parsing yang lebih lama.

### Implementasi dalam OmniCraft

Bayangkan Anda punya komponen dengan fitur yang tidak terpakai:

```omni
<script>
  const count = signal(0);
  const name = signal("Alice");  // Tidak pernah digunakan!
  
  function increment() {
    count.set(count() + 1);
  }
  
  function decrement() {  // Tidak pernah dipanggil!
    count.set(count() - 1);
  }
  
  function reset() {  // Tidak pernah dipanggil!
    count.set(0);
  }
</script>

<canvas>
  <text content={`Count: ${count()}`} />
  <rect @click={increment} />
</canvas>
```

Kompilator yang pintar akan melakukan analisis dan mendeteksi bahwa `name`, `decrement`, dan `reset` tidak pernah digunakan:

```rust
// Setelah Dead Code Elimination
pub struct Component {
    count: Signal<i32>,  // Kept - used in template
    // name: REMOVED - never accessed
}

impl Component {
    fn increment(&mut self) {  // Kept - referenced in @click
        self.count.set(self.count.get() + 1);
    }
    
    // decrement: REMOVED - never called
    // reset: REMOVED - never called
}
```

Implementasi DCE:

```rust
// compiler/src/optimizer/dead_code_elimination.rs

pub struct DeadCodeEliminator {
    used_symbols: HashSet<String>,
    call_graph: HashMap<String, Vec<String>>,
}

impl DeadCodeEliminator {
    pub fn eliminate(&mut self, component: &mut Component) {
        // Step 1: Mark phase - tandai semua yang digunakan
        self.mark_used_from_template(&component.template);
        self.mark_used_from_events(&component.template);
        
        // Step 2: Sweep phase - hapus yang tidak ditandai
        self.remove_unused_signals(component);
        self.remove_unused_functions(component);
        self.remove_unused_imports(component);
    }
    
    fn mark_used_from_template(&mut self, template: &Template) {
        // Scan template untuk identifier yang digunakan
        for node in &template.children {
            match node {
                Node::Element { attributes, .. } => {
                    for attr in attributes {
                        if let AttributeValue::Dynamic(expr) = &attr.value {
                            self.mark_used_in_expression(expr);
                        }
                    }
                }
                Node::Text { content } => {
                    self.mark_used_in_expression(content);
                }
                _ => {}
            }
        }
    }
    
    fn mark_used_in_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Call { callee, args } => {
                if let Expression::Identifier(name) = &**callee {
                    self.used_symbols.insert(name.clone());
                    
                    // Jika ini signal call, mark signal sebagai used
                    if self.is_signal(name) {
                        self.used_symbols.insert(name.clone());
                    }
                }
                
                // Rekursif untuk arguments
                for arg in args {
                    self.mark_used_in_expression(arg);
                }
            }
            
            Expression::Identifier(name) => {
                self.used_symbols.insert(name.clone());
            }
            
            Expression::Binary { left, right, .. } => {
                self.mark_used_in_expression(left);
                self.mark_used_in_expression(right);
            }
            
            _ => {}
        }
    }
    
    fn remove_unused_functions(&mut self, component: &mut Component) {
        if let Some(script) = &mut component.script {
            script.statements.retain(|stmt| {
                match stmt {
                    Statement::FunctionDeclaration { name, .. } => {
                        // Keep hanya jika used
                        self.used_symbols.contains(name)
                    }
                    _ => true
                }
            });
        }
    }
}
```

---

## 3. **Loop Unrolling** 🔄

### Apa Itu?

Loop Unrolling adalah teknik di mana kompilator "membuka" loop dengan menduplikasi body loop beberapa kali, mengurangi atau menghilangkan overhead loop control (checking condition, incrementing counter, jumping).

Contoh sederhana:

```javascript
// Original loop
for (let i = 0; i < 4; i++) {
    process(i);
}
```

Setiap iterasi loop ada overhead:
1. Check condition `i < 4`
2. Execute body `process(i)`
3. Increment `i++`
4. Jump back to start

Setelah unrolling:

```javascript
// Unrolled - no loop overhead!
process(0);
process(1);
process(2);
process(3);
```

### Kapan Loop Unrolling Efektif?

Loop unrolling paling efektif untuk:

**Small loop counts** - Loop yang iterasinya sedikit dan diketahui compile-time:

```omni
{#each [1, 2, 3, 4] as item}
  <circle x={item * 100} y={300} radius={30} />
{/each}
```

Ini bisa di-unroll menjadi:

```rust
// Unrolled
world.spawn_circle(100.0, 300.0, 30.0);
world.spawn_circle(200.0, 300.0, 30.0);
world.spawn_circle(300.0, 300.0, 30.0);
world.spawn_circle(400.0, 300.0, 30.0);
```

**Hot paths** - Loop yang dieksekusi sangat sering (setiap frame):

```rust
// Original - loop overhead setiap frame
for entity in &entities {
    entity.update(delta_time);
}

// Unrolled untuk 4 entities
entities[0].update(delta_time);
entities[1].update(delta_time);
entities[2].update(delta_time);
entities[3].update(delta_time);
```

### Partial Unrolling

Untuk loop besar, kompilator bisa melakukan partial unrolling:

```rust
// Original - 1000 iterations
for (let i = 0; i < 1000; i++) {
    process(i);
}

// Partially unrolled - 250 iterations, 4 operations each
for (let i = 0; i < 1000; i += 4) {
    process(i);
    process(i + 1);
    process(i + 2);
    process(i + 3);
}
// Loop overhead reduced by 75%!
```

### Implementasi dalam OmniCraft

```rust
// compiler/src/optimizer/loop_unrolling.rs

pub struct LoopUnroller {
    max_unroll_count: usize,  // Maksimal berapa kali unroll
}

impl LoopUnroller {
    pub fn unroll_loop(&self, loop_node: &EachBlock) -> Option<Vec<Node>> {
        // Cek apakah array length diketahui compile-time
        let array_length = self.get_static_array_length(&loop_node.expression)?;
        
        if array_length > self.max_unroll_count {
            // Terlalu besar untuk unroll
            return None;
        }
        
        // Unroll loop
        let mut unrolled_nodes = Vec::new();
        
        for i in 0..array_length {
            // Clone body dan substitute index
            for node in &loop_node.body {
                let substituted = self.substitute_loop_variable(
                    node, 
                    &loop_node.binding, 
                    i
                );
                unrolled_nodes.push(substituted);
            }
        }
        
        Some(unrolled_nodes)
    }
    
    fn get_static_array_length(&self, expr: &Expression) -> Option<usize> {
        match expr {
            Expression::Literal(Literal::Array(items)) => {
                Some(items.len())
            }
            Expression::Identifier(name) => {
                // Lookup constant array
                self.constants.get(name)
                    .and_then(|val| val.as_array())
                    .map(|arr| arr.len())
            }
            _ => None
        }
    }
}
```

Contoh hasil:

```omni
<!-- Input -->
{#each [10, 20, 30] as size}
  <circle radius={size} fill="#00d4ff" />
{/each}

<!-- Output after unrolling -->
<circle radius={10} fill="#00d4ff" />
<circle radius={20} fill="#00d4ff" />
<circle radius={30} fill="#00d4ff" />
```

---

## 4. **Common Subexpression Elimination (CSE)** 🔍

### Apa Itu?

CSE mendeteksi ekspresi yang dihitung lebih dari sekali dengan hasil yang sama, lalu menyimpan hasilnya dan menggunakan kembali nilai tersimpan.

Contoh:

```javascript
// Original - redundant calculations
const a = x * y + z;
const b = x * y - z;  // x * y dihitung lagi!
const c = x * y * 2;  // x * y dihitung lagi!
```

Kompilator mengenali bahwa `x * y` muncul tiga kali:

```javascript
// After CSE
const temp = x * y;    // Hitung sekali saja
const a = temp + z;
const b = temp - z;
const c = temp * 2;
```

### Dalam Konteks Visual Computing

CSE sangat powerful untuk transformasi geometri:

```omni
<script>
  const angle = signal(45);
  const radius = signal(100);
</script>

<canvas>
  <!-- Sin dan Cos dihitung berkali-kali! -->
  <circle 
    x={400 + Math.cos(angle() * Math.PI / 180) * radius()} 
    y={300 + Math.sin(angle() * Math.PI / 180) * radius()}
  />
  
  <line 
    x1={400}
    y1={300}
    x2={400 + Math.cos(angle() * Math.PI / 180) * radius()}
    y2={300 + Math.sin(angle() * Math.PI / 180) * radius()}
  />
</canvas>
```

Tanpa CSE, `Math.cos()` dan `Math.sin()` dipanggil total 4 kali! Dengan CSE:

```rust
// Generated with CSE
let angle_rad = self.angle.get() * std::f32::consts::PI / 180.0;  // Compute once
let cos_angle = angle_rad.cos();  // Compute once
let sin_angle = angle_rad.sin();  // Compute once
let r = self.radius.get();

let offset_x = cos_angle * r;  // Reuse
let offset_y = sin_angle * r;  // Reuse

// Circle
circle_transform.position = Vec2::new(400.0 + offset_x, 300.0 + offset_y);

// Line
line.end = Vec2::new(400.0 + offset_x, 300.0 + offset_y);
```

Implementasi:

```rust
// compiler/src/optimizer/cse.rs

pub struct CommonSubexpressionEliminator {
    expressions: HashMap<ExpressionHash, String>,  // Map expression -> temp variable
    temp_counter: usize,
}

impl CommonSubexpressionEliminator {
    pub fn eliminate(&mut self, statements: &mut Vec<Statement>) -> Vec<Statement> {
        let mut new_statements = Vec::new();
        
        for stmt in statements {
            self.process_statement(stmt, &mut new_statements);
        }
        
        new_statements
    }
    
    fn process_statement(&mut self, stmt: &Statement, output: &mut Vec<Statement>) {
        // Scan untuk subexpressions
        let subexprs = self.find_subexpressions(stmt);
        
        // Untuk setiap subexpression yang muncul > 1 kali
        for (expr, count) in subexprs {
            if count > 1 {
                // Generate temp variable
                let temp_name = format!("_cse_temp_{}", self.temp_counter);
                self.temp_counter += 1;
                
                // Store hasil di temp variable
                output.push(Statement::VariableDeclaration {
                    kind: VarKind::Const,
                    name: temp_name.clone(),
                    init: Some(expr.clone()),
                    reactive: false,
                });
                
                // Remember mapping
                let hash = self.hash_expression(&expr);
                self.expressions.insert(hash, temp_name);
            }
        }
        
        // Replace subexpressions dengan temp variables
        let replaced = self.replace_subexpressions(stmt);
        output.push(replaced);
    }
    
    fn hash_expression(&self, expr: &Expression) -> ExpressionHash {
        // Create hash berdasarkan structure expression
        // Expressions yang semantically equivalent punya hash sama
        // Misalnya: x + y == x + y, tapi x + y != y + x (order matters)
        
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        self.structural_hash(expr, &mut hasher);
        ExpressionHash(hasher.finish())
    }
}
```

---

## 5. **Strength Reduction** 💪

### Apa Itu?

Strength Reduction mengganti operasi yang "mahal" dengan operasi yang lebih murah tapi hasilnya sama. Operasi berbeda memiliki "cost" berbeda dalam CPU cycles:

```
Operation Cost (approximate CPU cycles):
Addition:       1-2 cycles
Subtraction:    1-2 cycles
Multiplication: 3-5 cycles
Division:       15-40 cycles
Modulo:         20-50 cycles
Power:          100+ cycles
Trigonometry:   100+ cycles
```

Contoh klasik:

```javascript
// Expensive
for (let i = 0; i < 1000; i++) {
    array[i] = i * 4;  // Multiplication setiap iterasi
}

// Cheaper - strength reduced
let temp = 0;
for (let i = 0; i < 1000; i++) {
    array[i] = temp;
    temp = temp + 4;  // Addition instead of multiplication!
}
```

Addition jauh lebih cepat daripada multiplication, jadi ini memberikan speedup signifikan dalam loop besar.

### Strength Reduction Patterns

**Pattern 1: Multiplication → Shift**

```rust
// Original
let x = y * 8;

// Reduced (8 = 2^3)
let x = y << 3;  // Bit shift jauh lebih cepat!
```

**Pattern 2: Division → Shift**

```rust
// Original
let x = y / 16;

// Reduced (16 = 2^4)
let x = y >> 4;
```

**Pattern 3: Power of 2 checks → Bit operations**

```rust
// Original
if (x % 2 == 0) { }  // Modulo expensive

// Reduced
if ((x & 1) == 0) { }  // Bitwise AND cheap
```

**Pattern 4: Expensive functions → Lookup tables**

```rust
// Original - sin() called every frame
let y = Math.sin(angle);

// Reduced - precomputed lookup table
static SIN_TABLE: [f32; 360] = [...];  // Precomputed at compile time
let y = SIN_TABLE[angle as usize];
```

### Implementasi dalam OmniCraft

```rust
// compiler/src/optimizer/strength_reduction.rs

pub struct StrengthReducer;

impl StrengthReducer {
    pub fn reduce(&self, expr: &Expression) -> Expression {
        match expr {
            Expression::Binary { left, op, right } => {
                match (left.as_ref(), op, right.as_ref()) {
                    // x * power_of_2 → x << log2(power_of_2)
                    (_, BinaryOp::Mul, Expression::Literal(Literal::Number(n))) 
                        if self.is_power_of_two(*n) => 
                    {
                        let shift_amount = n.log2() as i32;
                        Expression::Binary {
                            left: left.clone(),
                            op: BinaryOp::LeftShift,
                            right: Box::new(Expression::Literal(
                                Literal::Number(shift_amount as f64)
                            )),
                        }
                    }
                    
                    // x / power_of_2 → x >> log2(power_of_2)
                    (_, BinaryOp::Div, Expression::Literal(Literal::Number(n))) 
                        if self.is_power_of_two(*n) => 
                    {
                        let shift_amount = n.log2() as i32;
                        Expression::Binary {
                            left: left.clone(),
                            op: BinaryOp::RightShift,
                            right: Box::new(Expression::Literal(
                                Literal::Number(shift_amount as f64)
                            )),
                        }
                    }
                    
                    // x % 2 → x & 1
                    (_, BinaryOp::Mod, Expression::Literal(Literal::Number(2.0))) => {
                        Expression::Binary {
                            left: left.clone(),
                            op: BinaryOp::BitwiseAnd,
                            right: Box::new(Expression::Literal(Literal::Number(1.0))),
                        }
                    }
                    
                    _ => expr.clone()
                }
            }
            _ => expr.clone()
        }
    }
    
    fn is_power_of_two(&self, n: f64) -> bool {
        n > 0.0 && (n as u32).is_power_of_two()
    }
}
```

---

## 6. **Vectorization (SIMD)** ⚡

### Apa Itu SIMD?

SIMD stands for Single Instruction, Multiple Data. Ini adalah kemampuan CPU modern untuk melakukan operasi yang sama pada banyak data sekaligus dalam satu instruksi CPU.

Bayangkan Anda punya 4 nilai yang perlu ditambah 10:

```rust
// Scalar (traditional) - 4 separate operations
let a = x1 + 10;
let b = x2 + 10;
let c = x3 + 10;
let d = x4 + 10;
// Takes 4 CPU cycles
```

Dengan SIMD:

```rust
// SIMD - 1 operation on 4 values simultaneously!
let [a, b, c, d] = simd_add([x1, x2, x3, x4], [10, 10, 10, 10]);
// Takes 1 CPU cycle - 4x speedup!
```

### Mengapa SIMD Penting untuk Graphics?

Rendering involves massive parallel data processing yang perfect untuk SIMD:

- **Transforming vertices**: Multiply 1000 vertices dengan transformation matrix
- **Calculating colors**: Blend 1000 pixels dengan alpha values  
- **Physics updates**: Update position untuk 1000 entities

Semua ini adalah operasi yang sama diulang pada banyak data - perfect candidate untuk SIMD!

### Auto-vectorization dalam Rust

Rust compiler (LLVM) bisa otomatis mendeteksi patterns yang bisa di-SIMD-kan:

```rust
// Original code
fn update_positions(positions: &mut [Vec2], velocities: &[Vec2], dt: f32) {
    for i in 0..positions.len() {
        positions[i].x += velocities[i].x * dt;
        positions[i].y += velocities[i].y * dt;
    }
}

// LLVM auto-vectorizes to (conceptually):
fn update_positions_vectorized(positions: &mut [Vec2], velocities: &[Vec2], dt: f32) {
    // Process 4 at a time using SIMD
    for chunk in positions.chunks_exact_mut(4) {
        let vel_chunk = velocities[..4];
        let dt_vec = [dt, dt, dt, dt];
        
        // Single SIMD instruction processes all 4 positions!
        simd_fma(chunk, vel_chunk, dt_vec);
    }
    
    // Handle remainder
    for i in (positions.len() / 4 * 4)..positions.len() {
        positions[i].x += velocities[i].x * dt;
        positions[i].y += velocities[i].y * dt;
    }
}
```

### Explicit SIMD untuk Maximum Performance

Untuk critical paths, kita bisa menggunakan explicit SIMD:

```rust
use std::simd::*;

// Explicit SIMD untuk maximum control
fn blend_colors_simd(
    dest: &mut [u32], 
    src: &[u32], 
    alpha: f32
) {
    let alpha_vec = f32x4::splat(alpha);
    let one_minus_alpha = f32x4::splat(1.0 - alpha);
    
    // Process 4 pixels at once
    for i in (0..dest.len()).step_by(4) {
        // Load 4 pixels
        let d = u32x4::from_slice(&dest[i..i+4]);
        let s = u32x4::from_slice(&src[i..i+4]);
        
        // Convert to float
        let d_f = d.cast::<f32>();
        let s_f = s.cast::<f32>();
        
        // Blend: result = src * alpha + dest * (1 - alpha)
        let blended = s_f * alpha_vec + d_f * one_minus_alpha;
        
        // Convert back and store
        let result = blended.cast::<u32>();
        result.copy_to_slice(&mut dest[i..i+4]);
    }
}
```

### Implementasi dalam OmniCraft Compiler

Kompilator bisa mengatur data layout dan generate code yang SIMD-friendly:

```rust
// compiler/src/optimizer/vectorization.rs

pub struct Vectorizer {
    target_simd_width: usize,  // 4 for SSE, 8 for AVX
}

impl Vectorizer {
    pub fn vectorize_loop(&self, loop_stmt: &Statement) -> Statement {
        // Analyze loop untuk vectorization potential
        if !self.is_vectorizable(loop_stmt) {
            return loop_stmt.clone();
        }
        
        // Transform to SIMD version
        self.generate_simd_code(loop_stmt)
    }
    
    fn is_vectorizable(&self, loop_stmt: &Statement) -> bool {
        // Check criteria:
        // 1. Loop iterates over array
        // 2. No dependencies between iterations
        // 3. Operations are SIMD-able (add, mul, etc)
        // 4. Array is properly aligned
        
        true  // Simplified
    }
    
    fn generate_simd_code(&self, loop_stmt: &Statement) -> Statement {
        // Generate code using simd intrinsics
        // ...
    }
}
```

### Data Layout untuk SIMD (Structure of Arrays)

SIMD works best dengan data layout yang contiguous. Ini disebut Structure of Arrays (SoA) vs Array of Structures (AoS):

```rust
// AoS - Bad for SIMD
struct Entity {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}
let entities: Vec<Entity> = vec![...];

// Memory layout: [x1, y1, vx1, vy1, x2, y2, vx2, vy2, ...]
// Hard to SIMD - data not contiguous!

// SoA - Good for SIMD
struct Entities {
    x: Vec<f32>,
    y: Vec<f32>,
    vx: Vec<f32>,
    vy: Vec<f32>,
}

// Memory layout:
// x:  [x1, x2,  x3,  x4, ...]
// y:  [y1,  y2,  y3,  y4, ...]
// vx: [vx1, vx2, vx3, vx4, ...]
// vy: [vy1, vy2, vy3, vy4, ...]
// Perfect for SIMD!
```

Bevy ECS yang digunakan OmniCraft sudah menggunakan SoA layout secara default, jadi kita automatically get SIMD benefits!

---

## 7. **Memory Layout Optimization** 🧠

### Cache Lines dan Memory Access Patterns

Modern CPUs memiliki hierarki memory:

```
CPU Registers:    ~1 cycle,    ~1 KB
L1 Cache:         ~4 cycles,   32-64 KB
L2 Cache:         ~12 cycles,  256 KB - 1 MB
L3 Cache:         ~40 cycles,  2-32 MB
RAM:              ~200 cycles, GBs
```

Ketika CPU mengakses memory, ia tidak hanya mengambil byte yang diminta, tapi mengambil seluruh **cache line** (64 bytes pada most CPUs). Ini berarti:

**Good memory access** (sequential):
```rust
// Array akses sequential - cache friendly
let mut sum = 0;
for i in 0..array.len() {
    sum += array[i];  // Each access loads next 64 bytes
}
// Fast - most data already in cache!
```

**Bad memory access** (random):
```rust
// Random access - cache unfriendly
for i in 0..array.len() {
    let index = random_indices[i];
    sum += array[index];  // Each access might miss cache
}
// Slow - constant cache misses!
```

### False Sharing

False sharing terjadi ketika multiple threads mengakses different data yang kebetulan berada di cache line yang sama:

```rust
// Bad - false sharing
struct Counter {
    thread1_count: AtomicU64,  // Offset 0
    thread2_count: AtomicU64,  // Offset 8
}
// Both in same cache line (64 bytes)!
// Thread 1 modifying thread1_count invalidates Thread 2's cache

// Good - no false sharing
#[repr(align(64))]  // Force 64-byte alignment
struct Counter {
    thread1_count: AtomicU64,
    _padding1: [u8; 56],  // Pad to 64 bytes
    thread2_count: AtomicU64,
    _padding2: [u8; 56],
}
// Now in separate cache lines - no interference!
```

### Implementasi dalam OmniCraft

OmniCraft compiler bisa mengatur struct layout untuk optimal cache performance:

```rust
// compiler/src/optimizer/memory_layout.rs

pub struct MemoryLayoutOptimizer;

impl MemoryLayoutOptimizer {
    pub fn optimize_struct(&self, struct_def: &Struct) -> Struct {
        let mut optimized = struct_def.clone();
        
        // Sort fields by size (largest first)
        // This minimizes padding
        optimized.fields.sort_by_key(|f| std::cmp::Reverse(f.size()));
        
        // Add padding to prevent false sharing for atomic fields
        for field in &mut optimized.fields {
            if field.is_atomic() {
                field.add_alignment(64);  // Cache line size
            }
        }
        
        optimized
    }
    
    pub fn suggest_soa_conversion(&self, struct_def: &Struct) -> Option<SoAStruct> {
        // Analyze if SoA would be beneficial
        if struct_def.array_usage_count > 1000 
            && struct_def.has_simd_operations() 
        {
            Some(self.convert_to_soa(struct_def))
        } else {
            None
        }
    }
}
```

---

## 8. **Branch Prediction Hints** 🔮

### Apa Itu Branch Prediction?

Modern CPUs menggunakan speculative execution - mereka "menebak" cabang mana yang akan diambil dalam conditional dan mulai mengeksekusi sebelum kondisi benar-benar dievaluasi. Jika tebakan salah, CPU harus membuang semua pekerjaan spekulatif dan mulai lagi (**branch misprediction penalty** ~10-20 cycles).

### Likely/Unlikely Hints

Kita bisa memberi hint ke CPU tentang cabang mana yang lebih probable:

```rust
// Without hints
if condition {  // 50/50 guess
    common_case();
} else {
    rare_case();
}

// With hints
if likely(condition) {  // Tell CPU: this is usually true
    common_case();
} else {
    rare_case();
}
```

Compiler implementation:

```rust
#[inline(always)]
pub fn likely(condition: bool) -> bool {
    // Use LLVM intrinsic untuk branch prediction
    #[cfg(target_feature = "llvm")]
    unsafe {
        std::intrinsics::likely(condition)
    }
    
    #[cfg(not(target_feature = "llvm"))]
    condition
}
```

### Branchless Code

Kadang lebih baik menghilangkan branch sama sekali dengan arithmetic tricks:

```rust
// With branch
let result = if x > 0 { a } else { b };

// Branchless (using arithmetic)
let mask = (x > 0) as i32;  // 1 if true, 0 if false
let result = a * mask + b * (1 - mask);
// No branch - always executes both paths but selects result arithmetically
```

Ini bagus untuk code yang **tidak predictable** (50/50 probability). Untuk predictable branches, branch prediction lebih baik.

### Implementasi dalam OmniCraft

```rust
// compiler/src/optimizer/branch_optimization.rs

pub struct BranchOptimizer {
    branch_statistics: HashMap<NodeId, BranchStats>,
}

struct BranchStats {
    true_count: usize,
    false_count: usize,
}

impl BranchOptimizer {
    pub fn optimize_branch(&self, if_node: &IfBlock) -> IfBlock {
        let stats = self.branch_statistics.get(&if_node.id);
        
        if let Some(stats) = stats {
            let true_probability = stats.true_count as f32 
                / (stats.true_count + stats.false_count) as f32;
            
            if true_probability > 0.9 {
                // Very likely to be true
                return self.add_likely_hint(if_node);
            } else if true_probability < 0.1 {
                // Very unlikely to be true
                return self.add_unlikely_hint(if_node);
            } else if (0.4..=0.6).contains(&true_probability) {
                // Unpredictable - consider branchless
                if self.is_branchless_beneficial(if_node) {
                    return self.convert_to_branchless(if_node);
                }
            }
        }
        
        if_node.clone()
    }
    
    fn is_branchless_beneficial(&self, if_node: &IfBlock) -> bool {
        // Branchless good for:
        // - Simple arithmetic in both branches
        // - No side effects
        // - Similar cost for both branches
        
        self.is_simple_arithmetic(&if_node.then_branch) 
            && self.is_simple_arithmetic(&if_node.else_branch)
            && !self.has_side_effects(if_node)
    }
}
```

---

## 9. **Function Call Optimization** 📞

### Tail Call Optimization (TCO)

Tail call adalah function call yang merupakan operasi terakhir dalam fungsi. Compiler bisa mengoptimalkan ini menjadi jump instead of call, menghindari stack growth:

```rust
// Recursive fibonacci - grows stack
fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)  // NOT tail call - addition after
    }
}

// Tail-recursive fibonacci
fn fibonacci_tail(n: u32, a: u32, b: u32) -> u32 {
    if n == 0 {
        a
    } else {
        fibonacci_tail(n - 1, b, a + b)  // TAIL CALL - last operation
    }
}

// After TCO, compiled to:
fn fibonacci_tail_optimized(mut n: u32, mut a: u32, mut b: u32) -> u32 {
    loop {
        if n == 0 {
            return a;
        }
        let temp = b;
        b = a + b;
        a = temp;
        n -= 1;
    }
}
// No recursion - converted to loop!
```

### Devirtualization

Mengubah dynamic dispatch (virtual function call through vtable) menjadi direct call:

```rust
trait Renderer {
    fn render(&self);
}

// Dynamic dispatch - slow
fn render_all(renderers: &[Box<dyn Renderer>]) {
    for r in renderers {
        r.render();  // Virtual call through vtable
    }
}

// After devirtualization (if compiler knows concrete type)
fn render_all_optimized(renderers: &[Box<CircleRenderer>]) {
    for r in renderers {
        CircleRenderer::render(r);  // Direct call - much faster!
    }
}
```

Compiler can do this when it can prove yang concrete type at compile time through whole-program analysis.

---

## 10. **Profile-Guided Optimization (PGO)** 📊

### Apa Itu PGO?

PGO adalah proses dua-langkah:

**Step 1: Instrumented Build**
```bash
# Build with instrumentation
cargo build --profile instrumented

# Run program (collects profiling data)
./target/instrumented/app

# Generates profile data file
```

**Step 2: Optimized Build**
```bash
# Build using profile data
cargo build --profile release --use-profile-data
```

Compiler menggunakan actual runtime data untuk membuat keputusan optimasi yang lebih baik:

- **Hot/cold code separation** - Letakkan hot code di cache lines yang sama
- **Better inline decisions** - Inline functions yang actually called frequently
- **Better branch predictions** - Optimize berdasarkan actual branch frequencies
- **Better register allocation** - Prioritize registers untuk hot variables

### Benefits

Studies menunjukkan PGO bisa memberikan **10-30% speedup** for compute-intensive workloads!

### Implementasi dalam OmniCraft CLI

```rust
// cli/src/commands/build.rs

pub async fn build_with_pgo() -> Result<()> {
    println!("🔬 Starting Profile-Guided Optimization build...");
    
    // Step 1: Instrumented build
    println!("Step 1/3: Building instrumented binary...");
    Command::new("cargo")
        .args(&["build", "--profile", "pgo-instrumented"])
        .env("RUSTFLAGS", "-Cprofile-generate=/tmp/pgo-data")
        .status()?;
    
    // Step 2: Run untuk collect profiling data
    println!("Step 2/3: Collecting profile data...");
    println!("Please run your app through typical usage scenarios.");
    println!("Press Enter when done...");
    std::io::stdin().read_line(&mut String::new())?;
    
    // Step 3: Merge profiling data
    println!("Step 3/3: Building optimized binary...");
    Command::new("llvm-profdata")
        .args(&["merge", "-o", "/tmp/pgo-data/merged.profdata", "/tmp/pgo-data"])
        .status()?;
    
    // Final build with PGO
    Command::new("cargo")
        .args(&["build", "--release"])
        .env("RUSTFLAGS", "-Cprofile-use=/tmp/pgo-data/merged.profdata")
        .status()?;
    
    println!("✅ PGO build complete!");
    Ok(())
}
```

---

## 📊 Perbandingan Impact Optimasi

| Optimization | Complexity | Impact | When to Use |
|--------------|-----------|--------|-------------|
| **Inlining** | Medium | 🔥🔥🔥🔥 High | Small functions, hot paths |
| **Constant Folding** | Low | 🔥🔥🔥🔥🔥 Very High | Always |
| **Dead Code Elimination** | Low | 🔥🔥🔥 Medium-High | Always |
| **Loop Unrolling** | Medium | 🔥🔥🔥 Medium-High | Small loops, hot paths |
| **CSE** | Medium | 🔥🔥🔥 Medium-High | Complex expressions |
| **Strength Reduction** | Low | 🔥🔥 Medium | Expensive operations in loops |
| **SIMD** | High | 🔥🔥🔥🔥🔥 Very High | Array operations, graphics |
| **Memory Layout** | Medium | 🔥🔥🔥🔥 High | Data-intensive code |
| **Branch Optimization** | Medium | 🔥🔥 Medium | Conditional-heavy code |
| **Tail Call Optimization** | Low | 🔥🔥 Medium | Recursive algorithms |
| **PGO** | High | 🔥🔥🔥🔥 High | Production builds |

---

## 🎯 Recommended Implementation Order untuk OmniCraft

### Phase 1: Foundation (Weeks 1-2)
1. ✅ **Constant Folding** - Easy, huge impact
2. ✅ **Dead Code Elimination** - Easy, good impact
3. ✅ **Inlining** - Core optimization

### Phase 2: Loop Optimizations (Weeks 3-4)
4. ✅ **Loop Unrolling** - Good for static arrays
5. ✅ **Strength Reduction** - Easy wins in loops
6. ✅ **CSE** - Eliminate redundant calculations

### Phase 3: Advanced (Weeks 5-8)
7. ✅ **SIMD/Vectorization** - Massive speedup for arrays
8. ✅ **Memory Layout Optimization** - Cache-friendly data structures
9. ✅ **Branch Optimization** - Fine-tuning

### Phase 4: Production (Weeks 9+)
10. ✅ **PGO** - Final 10-30% speedup
11. ✅ **Link-Time Optimization (LTO)** - Cross-module optimization

---

Dengan mengimplementasikan semua optimasi ini secara sistematis, OmniCraft bisa mencapai performa yang mendekati hand-written assembly untuk banyak use cases, sambil tetap menyediakan high-level DSL yang developer-friendly!
