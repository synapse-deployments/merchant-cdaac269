use shopify_function::prelude::*;
use shopify_function::Result;

#[typegen("schema.graphql")]
pub mod schema {
  #[query("src/run.graphql")]
  pub mod run {}
}

fn main() {
  log!("Invoke a named export");
  std::process::abort();
}

#[shopify_function]
fn run(input: schema::run::Input) -> Result<schema::FunctionRunResult> {
  // Default empty result to return when no modifications are needed
  let default_result: schema::FunctionRunResult = Default::default();

  // Convert typed input to a generic Value for flexible inspection
  let input_value = match shopify_function::to_value(&input) {
    Ok(v) => v,
    Err(err) => {
      log!("Failed to serialize input: {}", err);
      return Ok(default_result);
    }
  };

  // Helper: try several common JSON paths for subtotal/total and return cents (i64)
  fn extract_subtotal_cents(v: &shopify_function::Value) -> Option<i64> {
    let paths: Vec<Vec<&str>> = vec![
      vec!["cart", "cost", "subtotalAmount", "amount"],
      vec!["cart", "subtotalPriceV2", "amount"],
      vec!["checkout", "subtotalPriceV2", "amount"],
      vec!["checkout", "totalPriceV2", "amount"],
    ];

    for path in paths {
      let mut cur = v;
      let mut ok = true;
      for key in &path {
        match cur.get(*key) {
          Some(next) => cur = next,
          None => { ok = false; break; }
        }
      }
      if !ok { continue; }

      if let Some(s) = cur.as_str() {
        if let Ok(decimal) = s.parse::<f64>() {
          return Some((decimal * 100.0).round() as i64);
        }
      } else if let Some(n) = cur.as_f64() {
        return Some((n * 100.0).round() as i64);
      }
    }

    None
  }

  let threshold_cents: i64 = 50 * 100; // $50.00

  match extract_subtotal_cents(&input_value) {
    Some(cents) if cents >= threshold_cents => {
      // Build delivery customization payload providing free shipping
      let mut price_obj = shopify_function::Value::object();
      price_obj.insert("amount".to_string(), shopify_function::Value::string("0.00"));

      // Try to preserve currency code from input if available, otherwise default to USD
      let currency_code = input_value
        .get("cart")
        .and_then(|c| c.get("cost"))
        .and_then(|cost| cost.get("subtotalAmount"))
        .and_then(|amt| amt.get("currencyCode"))
        .and_then(|cc| cc.as_str())
        .unwrap_or("USD");
      price_obj.insert("currencyCode".to_string(), shopify_function::Value::string(currency_code));

      let mut option_obj = shopify_function::Value::object();
      option_obj.insert("id".to_string(), shopify_function::Value::string("free-shipping"));
      option_obj.insert("title".to_string(), shopify_function::Value::string("Free Shipping"));
      option_obj.insert("price".to_string(), price_obj);

      let mut options_arr = shopify_function::Value::array();
      options_arr.push(option_obj);

      let mut group_obj = shopify_function::Value::object();
      group_obj.insert("options".to_string(), options_arr);

      let mut groups_arr = shopify_function::Value::array();
      groups_arr.push(group_obj);

      let mut result_obj = shopify_function::Value::object();
      result_obj.insert("deliveryGroups".to_string(), groups_arr);

      let json_value = match shopify_function::to_value(&result_obj) {
        Ok(j) => Some(j),
        Err(err) => {
          log!("Failed to serialize result payload: {}", err);
          None
        }
      };

      return Ok(schema::FunctionRunResult { json: json_value, ..Default::default() });
    }
    Some(_) => {
      // Subtotal present but below threshold: return default empty result
      return Ok(default_result);
    }
    None => {
      log!("Subtotal not found in input; returning default result.");
      return Ok(default_result);
    }
  }
}
