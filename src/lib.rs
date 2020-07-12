use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

#[macro_use]
extern crate lazy_static;

mod app_state;
mod common_funcs;
mod constants;
mod gl_setup;
mod programs;
mod shaders;
mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Binding JS involves a bit of boilerplate because we have to specify each name
// and signature to bind.
#[wasm_bindgen]
extern "C" {
    // Bindings must be named as their JS equivalent
    fn alert(s: &str);

    // A different name can be specified as long as the original name is passed to the macro.
    #[wasm_bindgen(js_name = prompt)]
    fn ask(s: &str) -> String;

    // Methods can be from any js namespace.
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // Using a different name allows us to specify various signatures.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_num(n: i32);

    //* JS Class example *\\
    // The process is a little verbose because create a binding for
    // each part of the class we want (class name, constructor, methods, setters, getters).
    type Coordinate;

    #[wasm_bindgen(constructor)]
    fn new(x: i32, y: i32) -> Coordinate;

    // methods must match the naming in the class declaration.
    #[wasm_bindgen(method)]
    fn printValues(this: &Coordinate) -> String;

    // getters are named as the property we want.
    #[wasm_bindgen(getter, method)]
    fn x(this: &Coordinate) -> i32;

    // setters are named the same as getters but with a `set_` prefix.
    #[wasm_bindgen(setter, method)]
    fn set_x(this: &Coordinate, x: i32);
}

#[wasm_bindgen]
pub fn manual_bindings_example() {
    alert("Hey buddy!");
    log(&ask("Tell me about your day!"));

    let coordinates = Coordinate::new(-4, 15);
    log_num(coordinates.x()); // prints -4

    coordinates.set_x(coordinates.x() * 2);
    log(&coordinates.printValues()); // prints (-8, 15)
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm with target web!");
}

// JS doesn't have a chars type which means:
// - The _c argument is the first char of a JS string.
// - The char returned will be a JS string.
#[wasm_bindgen]
pub fn char_example(_c: char) -> char {
    '🚀'
}

#[wasm_bindgen]
pub fn string_example(s: String) -> String {
    format!("Hello {}", s)
}

// str cannot be used as a return type.
// This is because we can't return borrowed references with the wasm_bindgen macro.
#[wasm_bindgen]
pub fn str_example(s: &str) -> String {
    format!("Hello {}", s)
}

#[wasm_bindgen]
pub fn number_example(n: i32) -> i32 {
    // assume the same for u32, usize, etc.
    n + 100
}

#[wasm_bindgen]
pub fn bool_example(_b: bool) -> bool {
    true
}

// `Box<[JsValue]>` are the representation for a JS array object.
// When it comes to Js Arrays:
// - They are iterable.
// - Can contain multiple types by being of type JsValue (strictly typed arrays exist for numbers).
// - Don't really support N-dimensional arrays and are expensive to work with.
#[wasm_bindgen]
pub fn mixed_array_example(array: Box<[JsValue]>) -> Box<[JsValue]> {
    for value in array.iter() {
        // compute things...
    }

    vec![
        "Hello".into(),
        512.into(),
        JsValue::NULL,
        JsValue::UNDEFINED,
        61.20.into(),
    ]
    .into_boxed_slice()
}

// Typed arrays are only available for number types.
// For example, the function below will return a JS Int32Array type.
#[wasm_bindgen]
pub fn typed_array_example(_array: Box<[i32]>) -> Box<[i32]> {
    vec![1, 2, 3, 4, 5, 6, 7].into_boxed_slice()
}

// When it comes to Option:
// - Some returns the value inside.
// - None returns a JS undefined.
#[wasm_bindgen(catch)]
pub fn option_example() -> Option<i32> {
    None
}

// When it comes to Result
// - Result<T, JsValue> is the only supported signature. T must be convertible to a JsValue.
// - #[wasm_bindgen(catch)] must be used when returning a result.
// - Err will be equivalent to a JS thrown error.
// - Ok will return the value inside.
#[wasm_bindgen]
pub fn result_example() -> Result<i32, JsValue> {
    // With the wasm prelude imported, we can convert most common types by calling .into()
    Err("Look Pa, I'm throwing a JS error!".into())
}
// - JS represents them through an object with a number for each variant.
#[wasm_bindgen]
pub enum ExampleEnum {
    Yes,
    No,
}

#[wasm_bindgen]
pub fn verify_enum_choice(choice: ExampleEnum) -> bool {
    match choice {
        ExampleEnum::Yes => true,
        ExampleEnum::No => false,
    }
}

// When it comes to Structs:
// - Cannot contain lifetimes or type parameters.
// - Each field value must impl the Copy trait.
#[wasm_bindgen]
pub struct ExampleStruct {
    pub value: i32,
}

// For struct impl, we have the option for struct methods and type-level functions.
// JS handles structs by creating a JS object with a pointer (i.o.w. we can use references!).
#[wasm_bindgen]
impl ExampleStruct {
    pub fn new(value: i32) -> ExampleStruct {
        ExampleStruct { value }
    }

    pub fn read_method(&self) -> i32 {
        self.value
    }

    pub fn write_method(&mut self, value: i32) {
        self.value = value;
    }

    pub fn transfer_ownership(self) -> ExampleStruct {
        self
    }
}

#[wasm_bindgen]
pub struct MyClient {
    gl: WebGlRenderingContext,
    program_color_2d: programs::Color2D,
    program_color_2d_gradient: programs::Color2DGradient,
}

#[wasm_bindgen]
impl MyClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let gl = gl_setup::initialize_webgl_context().unwrap();

        Self {
            program_color_2d: programs::Color2D::new(&gl),
            program_color_2d_gradient: programs::Color2DGradient::new(&gl),
            gl: gl,
        }
    }

    pub fn update(&mut self, time: f32, height: f32, width: f32) -> Result<(), JsValue> {
        app_state::update_dynamic_data(time, height, width);
        Ok(())
    }

    pub fn render(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        let curr_state = app_state::get_curr_state();

        self.program_color_2d.render(
            &self.gl,
            curr_state.control_bottom,
            curr_state.control_top,
            curr_state.control_left,
            curr_state.control_right,
            curr_state.canvas_height,
            curr_state.canvas_width,
        );

        self.program_color_2d_gradient.render(
            &self.gl,
            curr_state.control_bottom + 20.,
            curr_state.control_top - 20.,
            curr_state.control_left + 20.,
            curr_state.control_right - 20.,
            curr_state.canvas_height,
            curr_state.canvas_width,
        );
    }
}
