from sympy import symbols, Eq, solve

# Original prices and quantities
prices = {
    7082.57: 11,
    6432.85: 8,
    12725.80: 4,
    8448.55: 5,
    8071.76: 5,
    3823.12: 2,
}

desired_sum = 36250000.00

mult = 134

price_symbols = symbols(' '.join(f'price{i}' for i in range(len(prices))))

total_sum_expr = sum(price * quantity * mult for price, quantity in zip(price_symbols, prices.values()))
equation = Eq(total_sum_expr, desired_sum)

solutions = solve(equation, price_symbols)

new_prices = {}
for i, price_symbol in enumerate(price_symbols):
    original_price = list(prices.keys())[i]
    new_prices[original_price] = solutions[i].evalf()

price_changes = {price: new_prices[price] - price for price in prices.keys()}

print("Price changes to achieve the desired sum:")
for price, change in price_changes.items():
    print(f"Original price: {price}, Change: {change:.2f}")
