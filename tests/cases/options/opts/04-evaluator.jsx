// The evaluator decides what folds into a template, so every operator it implements is
// compared against Babel's own folding here — these were written from the spec, not observed.
const arith = <div a={1 + 2} b={10 - 3} c={4 * 5} d={9 / 3} e={7 % 4} f={2 ** 10} />
const compare = <div a={1 < 2 ? "lt" : "ge"} b={2 <= 2 ? "le" : "gt"} c={3 > 4 ? "gt" : "le"}
                     d={4 >= 4 ? "ge" : "lt"} e={3 === 3 ? "eq" : "ne"} f={3 !== 4 ? "ne" : "eq"} />
const strings = <div a={"a" < "b" ? "s" : "t"} b={"x" == "x" ? "eq" : "ne"} c={"1" == 1 ? "loose" : "strict"} />
const bits = <div a={5 & 3} b={5 | 3} c={5 ^ 3} d={1 << 4} e={-16 >> 2} f={-16 >>> 28} />
const unary = <div a={-5} b={+"3"} c={!0 ? "y" : "n"} d={typeof "x"} e={typeof 1} f={~5} />
const coercion = <div a={1 + "x"} b={true + 1} c={null + 1} d={"5" * 2} e={"" + false} />
const methods = <div a={"  x  ".trimStart()} b={"  x  ".trimEnd()} c={"a".concat("b", 1)} d={(255).toString()} />
const nested = <div a={(1 + 2) * 3 - 4 / 2} b={"n" + (1 < 2 ? 1 : 2)} />
