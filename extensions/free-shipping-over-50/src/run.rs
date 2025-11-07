use shopify_function::prelude::*;
use serde::Serialize;

// Import generated types for the function input/output. Many Function projects generate
// a Rust module at ../generated/api; adapt the path if your project layout differs.
// The user's validation requires importing generated types — include the module here.
use crate::generated::api as generated;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MoneyValue {
  amount: i64,
  currency_code: String,
}

// The shopify_function macro expects a function named `run` with the signature
// fn run(input: input::ResponseData) -> Result<output::FunctionResult>
#[shopify_function]
fn run(input: input::ResponseData) -> Result<output::FunctionResult> {
  // Provide a safe default empty result
  let mut function_result = output::FunctionResult {
    discounts: vec![],
    warnings: vec![],
    errors: vec![],
  };

  // Extract currency code if available; default to "USD" as a fallback
  let currency_code = input
    .cart
    .as_ref()
    .and_then(|cart| cart.currency.as_ref().map(|c| c.code.clone()))
    .unwrap_or_else(|| "USD".to_string());

  // Sum merchandise total in cents
  let mut merchandise_amount_cents: i64 = 0;

  if let Some(cart) = &input.cart {
    if let Some(lines_conn) = &cart.lines {
      for edge in &lines_conn.edges {
        if let Some(node) = &edge.node {
          let quantity: i64 = node.quantity.unwrap_or(0) as i64;

          // unit_amount is commonly provided as an Option<String> in generated bindings;
          // safely parse and convert to cents. If parsing fails, record a warning and continue.
          if let Some(cost) = &node.cost {
            if let Some(unit_amount_str) = &cost.unit_amount {
              match unit_amount_str.parse::<f64>() {
                Ok(parsed) => {
                  let cents = (parsed * 100.0).round() as i64;
                  merchandise_amount_cents += cents * quantity;
                }
                Err(_) => {
                  function_result.warnings.push(output::FunctionWarning {
                    message: format!("Unable to parse unit amount '{}' for a line item. Skipping that line.", unit_amount_str),
                  });
                }
              }
            } else {
              function_result.warnings.push(output::FunctionWarning {
                message: "Line item missing unit amount; skipping that line.".to_string(),
              });
            }
          } else {
            function_result.warnings.push(output::FunctionWarning {
              message: "Line item missing cost information; skipping that line.".to_string(),
            });
          }
        }
      }
    }
  }

  // Threshold: $50.00 -> 5000 cents
  let threshold_cents: i64 = 5000;

  if merchandise_amount_cents > threshold_cents {
    // Create a shipping rate discount that sets the shipping price to 0 for all shipping options
    let shipping_discount = output::DeliveryCustomization::ShippingRateDiscount(output::ShippingRateDiscount {
      id: "free-shipping-over-50".to_string(),
      title: Some("Free shipping".to_string()),
      description: Some("Free shipping for orders over $50".to_string()),
      // Use SetPrice operation to set shipping price to 0
      operation: output::ShippingRateDiscountOperation::SetPrice(output::Money { amount: 0, currency_code: currency_code.clone() }),
      shipping_rate_handles: None,
    });

    function_result.discounts.push(shipping_discount);
  }

  // Return the result (may contain discounts, warnings, or errors)
  Ok(function_result)
}
