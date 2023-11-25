function mulberry32(a: number) {
  return function () {
    let t = (a += 0x6d2b79f5);
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return Math.floor((((t ^ (t >>> 14)) >>> 0) / 4294967296) * 10);
  };
}

export function getGradientFromNumber(n: number) {
  const next = mulberry32(n);

  const colors = [
    "grape",
    "violet",
    "indigo",
    "cyan",
    "teal",
    "green",
    "orange",
    "blue",
    "orange",
    "blue",
  ];

  return { from: colors[next()], to: colors[next()], deg: 90 };
}
