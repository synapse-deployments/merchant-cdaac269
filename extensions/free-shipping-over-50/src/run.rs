import type { RunInput, FunctionRunResult, DeliveryCustomization, DeliveryOptionModification, MoneyV2 } from "../generated/api";

// Threshold in cents: $50.00
const THRESHOLD_CENTS = 50 * 100;

// Helper to parse a decimal money string (e.g. "52.34") into cents as integer
function parseAmountToCents(amountStr: string | undefined | null): number {
  if (!amountStr) return 0;
  const s = amountStr.trim();
  if (s.length === 0) return 0;

  // Handle optional negative sign
  const negative = s.startsWith("-");
  const normalized = negative ? s.slice(1) : s;

  const parts = normalized.split(".");
  const dollarsPart = parts[0] || "0";
  const centsPart = parts[1] || "";

  const dollars = parseInt(dollarsPart, 10) || 0;
  let cents = 0;
  if (centsPart.length === 0) {
    cents = 0;
  } else if (centsPart.length === 1) {
    cents = parseInt(centsPart, 10) * 10;
  } else {
    // take at most two digits, ignore extra precision (truncate)
    cents = parseInt(centsPart.slice(0, 2), 10);
  }

  const total = dollars * 100 + cents;
  return negative ? -total : total;
}

// Default empty result per recommendations
function emptyResult(): FunctionRunResult {
  return {
    customizations: null,
    errors: [],
  };
}

export function run(input): FunctionRunResult {
  try {
    // The generated input types provide access to cart and cost information
    const cart = input.cart ?? null;
    // Fallback currency
    const currency = cart?.currency ?? "USD";

    // Many generated schemas provide totalAmount as an object with amount (string) and currencyCode
    // Try multiple common paths to find a subtotal/total amount string
    let amountStr: string | undefined | null = null;

    if (cart && cart.cost && cart.cost.totalAmount && typeof cart.cost.totalAmount.amount === "string") {
      amountStr = cart.cost.totalAmount.amount;
    } else if (cart && cart.subtotalPrice && typeof cart.subtotalPrice === "string") {
      // older/generated variations
      amountStr = cart.subtotalPrice;
    } else if (cart && cart.totalPrice && typeof cart.totalPrice === "string") {
      amountStr = cart.totalPrice;
    }

    const subtotalCents = parseAmountToCents(amountStr);

    // If below threshold, return empty result (no customizations)
    if (subtotalCents < THRESHOLD_CENTS) {
      return emptyResult();
    }

    // Build a delivery customization that sets shipping price to 0 for options
    const zeroMoney: MoneyV2 = {
      amount: "0.00",
      currencyCode: currency,
    };

    const shippingModification: DeliveryOptionModification = {
      // Apply to all options by not specifying an id
      price: zeroMoney,
      title: "Free shipping",
      description: "Free shipping for orders over $50",
    };

    const deliveryCustomization: DeliveryCustomization = {
      modifications: [shippingModification],
    };

    const result: FunctionRunResult = {
      customizations: {
        delivery: deliveryCustomization,
      },
      errors: [],
    };

    return result;
  } catch (e) {
    // On unexpected errors, return a safe empty result plus a structured error if possible
    const message = e instanceof Error ? e.message : String(e);
    return {
      customizations: null,
      errors: [
        {
          message: `Free shipping function error: ${message}`,
        },
      ],
    };
  }
}
