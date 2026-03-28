function rotl(x: number, k: number) {
  return ((x << k) | (x >>> (32 - k))) >>> 0;
}

export function rng(seed: number) {
  let sm = BigInt(seed >>> 0);

  function splitmix64() {
    sm = BigInt.asUintN(64, sm + 0x9e3779b97f4a7c15n);
    let z = sm;
    z = BigInt.asUintN(64, (z ^ (z >> 30n)) * 0xbf58476d1ce4e5b9n);
    z = BigInt.asUintN(64, (z ^ (z >> 27n)) * 0x94d049bb133111ebn);
    return BigInt.asUintN(64, z ^ (z >> 31n));
  }

  const sm0 = splitmix64();
  const sm1 = splitmix64();

  let s0 = Number(BigInt.asUintN(32, sm0));
  let s1 = Number(BigInt.asUintN(32, sm0 >> 32n));
  let s2 = Number(BigInt.asUintN(32, sm1));
  let s3 = Number(BigInt.asUintN(32, sm1 >> 32n));

  return function () {
    const result = Math.imul(rotl(Math.imul(s1, 5) >>> 0, 7), 9) >>> 0;

    const t = (s1 << 9) >>> 0;

    s2 ^= s0;
    s3 ^= s1;
    s1 ^= s2;
    s0 ^= s3;

    s2 ^= t;
    s3 = rotl(s3, 11);

    return result / 0x100000000;
  };
}
