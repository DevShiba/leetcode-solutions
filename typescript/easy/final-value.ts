function finalValueAfterOperations(operations: string[]): number {
  let res = 0;
  for (const i of operations) {
    if (i === "X++" || i === "++X") {
      res++;
    } else if (i === "X--" || i === "--X") {
      res--;
    }
  }

  return res;
}
