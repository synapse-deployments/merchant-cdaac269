import type { RunInput, FunctionRunResult } from "../generated/api";

export function run(input): FunctionRunResult {
  // Default empty result
  const emptyResult: FunctionRunResult = { operations: [] };

  // Safely navigate input to get subtotal amount and deliveries
  // The generated RunInput shape for delivery customization includes:
  // - cartDeliveryCustomization.cartCost.subtotalAmount.amount (string)
  // - cartDeliveryCustomization.cartCost.subtotalAmount.currencyCode (string)
  // - cartDeliveryCustomization.deliveries[] with id fields

  const customization = (input as any).cartDeliveryCustomization;
  if (!customization) return emptyResult;

  const cartCost = customization.cartCost;
  if (!cartCost || !cartCost.subtotalAmount) return emptyResult;

  const subtotalAmountStr = cartCost.subtotalAmount.amount;
  const currency = cartCost.subtotalAmount.currencyCode || "USD";
  const subtotal = parseFloat(subtotalAmountStr || "0");

  const threshold = 50.0;

  if (isNaN(subtotal) || subtotal < threshold) {
    return emptyResult;
  }

  const deliveries = Array.isArray(customization.deliveries) ? customization.deliveries : [];
  const adjustments = deliveries
    .filter((d) => d && d.id)
    .map((d) => ({
      id: d.id,
      price: {
        amount: "0.00",
        currencyCode: currency,
      },
    }));

  if (adjustments.length === 0) return emptyResult;

  const result: FunctionRunResult = {
    operations: [
      {
        type: "setDeliveryPrices",
        deliveries: adjustments,
      },
    ],
  };

  return result;
}
