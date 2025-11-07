import type { RunInput, FunctionRunResult, Operation, Money } from "../generated/api";

// This function grants free shipping when cart subtotal (items + discounts, excluding shipping & taxes)
// exceeds $50.00. It expects amounts in minor units (e.g., cents) as provided in the generated API.

export function run(input): FunctionRunResult {
  // Default empty result
  const emptyResult: FunctionRunResult = {
    operations: [],
  };

  try {
    // Attempt to read checkout subtotal amount if provided
    let subtotalAmount: number | null = null;
    let currencyCode: string | null = null;

    if (input.checkout) {
      const checkout = input.checkout;

      if (checkout.subtotalAmount && typeof checkout.subtotalAmount.amount === "number") {
        subtotalAmount = checkout.subtotalAmount.amount;
        currencyCode = checkout.subtotalAmount.currencyCode || null;
      }

      // If subtotal not present, compute from line items' discountedTotalPrice
      if (subtotalAmount === null && checkout.lineItems) {
        let sum = 0;
        let foundCurrency: string | null = null;

        for (const edge of checkout.lineItems.edges || []) {
          const node = edge && edge.node;
          if (!node) continue;

          if (node.discountedTotalPrice && typeof node.discountedTotalPrice.amount === "number") {
            sum += node.discountedTotalPrice.amount;
            if (!foundCurrency && node.discountedTotalPrice.currencyCode) {
              foundCurrency = node.discountedTotalPrice.currencyCode;
            }
          } else if (node.originalTotalPrice && typeof node.originalTotalPrice.amount === "number") {
            // Fallback to originalTotalPrice if discounted not available
            sum += node.originalTotalPrice.amount;
            if (!foundCurrency && node.originalTotalPrice.currencyCode) {
              foundCurrency = node.originalTotalPrice.currencyCode;
            }
          }
        }

        if (sum > 0) {
          subtotalAmount = sum;
          currencyCode = foundCurrency;
        }
      }
    }

    // If we couldn't determine subtotal or currency, return empty result
    if (subtotalAmount === null || currencyCode === null) {
      return emptyResult;
    }

    // Threshold: $50.00 -> 5000 minor units (assumes 2 decimal currency)
    const thresholdMinorUnits = 5000;

    if (subtotalAmount >= thresholdMinorUnits) {
      // Build operation to set shipping price to 0 for all shipping lines
      const zeroMoney: Money = { amount: 0, currencyCode };

      const operation: Operation = {
        setShippingLinePrice: {
          price: zeroMoney,
          // applyToAllShippingLines indicates this applies to all applicable shipping lines
          applyToAllShippingLines: true,
        },
      };

      const result: FunctionRunResult = {
        operations: [operation],
      };

      return result;
    }

    // Not eligible for free shipping
    return emptyResult;
  } catch (e) {
    // On error, return empty result to avoid blocking checkout.
    return {
      operations: [],
    };
  }
}
