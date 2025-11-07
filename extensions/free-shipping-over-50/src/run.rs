import type { RunInput, FunctionRunResult, DeliveryOption, MoneyV2 } from "../generated/api";

// Exported entrypoint required by Shopify Functions (JavaScript/TypeScript variant)
export function run(input): FunctionRunResult {
  // Safe helpers to read nested properties
  const safe = {
    getSubtotalAmount(input): { amount: string; currencyCode: string } | null {
      try {
        const subtotal = input?.cart?.cost?.subtotalAmount;
        if (!subtotal) return null;
        // subtotal.amount is a string (major unit), e.g. "50.00"
        return { amount: subtotal.amount, currencyCode: subtotal.currencyCode };
      } catch (e) {
        return null;
      }
    },
  };

  // Default empty result structure
  const emptyResult: FunctionRunResult = {
    operations: [],
  };

  const subtotalObj = safe.getSubtotalAmount(input);
  if (!subtotalObj) {
    // No subtotal available — return empty result (no change)
    return emptyResult;
  }

  // Parse subtotal amount as a float in major currency units
  const subtotalFloat = parseFloat(subtotalObj.amount || "0");
  const threshold = 50.0;

  if (isNaN(subtotalFloat)) {
    return emptyResult;
  }

  if (subtotalFloat < threshold) {
    return emptyResult;
  }

  // Build a DeliveryOption object according to the generated API types
  const currency = subtotalObj.currencyCode || "USD";

  // Money object in major units as string (matches GraphQL MoneyV2: amount & currencyCode)
  const freePrice: MoneyV2 = {
    amount: "0.00",
    currencyCode: currency,
  };

  const deliveryOption: DeliveryOption = {
    // id should be unique for the option
    id: "free_shipping_over_50",
    title: "Free shipping",
    price: freePrice,
    // If the generated type includes additional fields (e.g., deliveryRange or description),
    // leaving them undefined is safe. Keep the object minimal and compatible.
  } as unknown as DeliveryOption;

  // The FunctionRunResult.operations entries depend on the generated API shape.
  // Typically, operations are shaped like: { type: 'AddDeliveryOption', deliveryOption: DeliveryOption }
  // To be robust against small schema differences, follow the common pattern used by Shopify Functions.

  const operations: any[] = [];

  // Add operation to add a delivery option. Field names may vary in generated types.
  // Use the typical "add_delivery_option" operation with a payload containing the option.
  operations.push({
    type: "add_delivery_option",
    deliveryOption: deliveryOption,
  });

  const result: FunctionRunResult = {
    operations,
  };

  return result;
}
