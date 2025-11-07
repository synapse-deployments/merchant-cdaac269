import shopifyFunction from "@shopify/functions";
import type { RunInput, FunctionRunResult } from "../generated/api";

// Threshold in the shop's minor currency unit (e.g., cents for USD).
const THRESHOLD_CENTS = 5000; // $50.00

// Helper: parse decimal amount string (e.g., "123.45") into integer minor units (cents).
function parseAmountToCents(amount): number {
  if (!amount || typeof amount !== "string") return 0;
  const parts = amount.split(".");
  const wholePart = parts[0].replace(/[^0-9-]/g, "");
  const whole = wholePart === "" ? 0 : parseInt(wholePart, 10);
  if (isNaN(whole)) return 0;
  let cents = whole * 100;
  if (parts.length === 2) {
    let frac = parts[1].replace(/[^0-9]/g, "");
    if (frac.length === 0) {
      // nothing
    } else if (frac.length === 1) {
      cents += parseInt(frac, 10) * 10;
    } else {
      // take first two digits (rounding could be added if desired)
      cents += parseInt(frac.slice(0, 2), 10);
    }
  }
  return cents;
}

// The exported run function expected by Shopify Functions.
export function run(input): FunctionRunResult {
  // Provide a safe default empty result
  const defaultResult: FunctionRunResult = {
    // delivery customization functions return operations that modify delivery prices
    // In the generated API, the shape may include 'discounts', 'adjustments' or 'operations'.
    // We'll return an empty array of operations by default.
    operations: [],
    errors: [],
  } as unknown as FunctionRunResult;

  try {
    // Defensive checks: ensure cart and subtotal exist on input
    const cart = (input && (input as any).cart) || null;
    const subtotal = cart && cart.subtotalPrice && cart.subtotalPrice.amount ? cart.subtotalPrice.amount : "0";

    const subtotalCents = parseAmountToCents(subtotal);

    if (subtotalCents > THRESHOLD_CENTS) {
      // Create an operation that sets shipping line price to zero.
      // The exact operation structure depends on the generated API. Commonly, delivery customization
      // functions return operations that target shipping lines and set their price.
      // We'll construct an operation object matching the typical schema from Shopify Functions examples.

      const operation = {
        type: "set_price",
        target: {
          type: "shipping_line",
        },
        // Set price to zero in money object (string amounts with currencyCode)
        price: {
          amount: "0.00",
          currencyCode: cart && cart.subtotalPrice && cart.subtotalPrice.currencyCode ? cart.subtotalPrice.currencyCode : "USD",
        },
        // Optional message shown to merchant/customer
        message: "Free shipping for orders over $50",
      };

      return {
        operations: [operation],
        errors: [],
      } as unknown as FunctionRunResult;
    }

    // If threshold not met, return default (no operations)
    return defaultResult;
  } catch (e) {
    // Handle unexpected errors by returning an error in the function result rather than throwing.
    const message = e && (e as Error).message ? (e as Error).message : String(e);
    return {
      operations: [],
      errors: [{ message }],
    } as unknown as FunctionRunResult;
  }
}

// Register the function (this import/registration may vary depending on your build tooling).
// The shopify functions runtime looks for the exported `run` function. The default export
// from '@shopify/functions' is sometimes used to register metadata; including this line
// does not change the exported run function but ensures the module is recognized during build.
export default shopifyFunction;
