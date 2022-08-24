use wasm_bindgen::JsValue;

pub async fn start() -> Result<(), JsValue> {
  let document = web_sys::window()
    .ok_or(JsValue::from("No Window"))?
    .document()
    .ok_or(JsValue::from("No Document"))?;
  let p = document.create_element("p")?;
  p.set_inner_html("Hello wasm!");
  let body = document.body().ok_or(JsValue::from("No Body"))?;
  body.append_child(&p)?;
  Ok(())
}
