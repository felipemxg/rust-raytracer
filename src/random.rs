//static mut KEY: u64 = 8589392;
static mut KEY: u32 = 8589392;
static mut CTR: u64 = 0;

#[inline(always)]
fn xorshift32(state: &mut u32) -> u32 {
    let mut x = *state;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    *state = x;
    x
}

#[inline(always)]
fn xorshift64(state: &mut u64) -> u64 {
    let mut x = *state;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *state = x;
    x
}

#[inline(always)]
fn squares64(ctr: u64, key: u64) -> u64 {
    let mut t = 0u64;
    let mut x = 0u64;
    let mut y = 0u64;
    let mut z = 0u64;
    x = ctr * key;
    y = x;
    z = y + key;
    x = x * x + y;
    x = (x >> 32) | (x << 32); /* round 1 */
    x = x * x + z;
    x = (x >> 32) | (x << 32); /* round 2 */
    x = x * x + y;
    x = (x >> 32) | (x << 32); /* round 3 */
    x = x * x + z;
    t = x;
    x = (x >> 32) | (x << 32); /* round 4 */
    return t ^ ((x * x + y) >> 32); /* round 5 */
}

#[inline(always)]
pub fn set_random_seed(seed: u32) {
    unsafe {
        KEY = seed;
    }
}

#[inline(always)]
pub fn randomf32() -> f32 {
    unsafe {
        //CTR += 1;
        //squares64(CTR, KEY) as f32 / (u64::MAX as f32 + 1.0)
        //xorshift64(&mut KEY) as f32 / (u64::MAX as f32 + 1.0)
        xorshift32(&mut KEY) as f32 / (u32::MAX as f32 + 1.0)
    }
}

#[inline(always)]
pub fn randomf32_range(min: f32, max: f32) -> f32 {
    min + (max - min) * randomf32()
}
