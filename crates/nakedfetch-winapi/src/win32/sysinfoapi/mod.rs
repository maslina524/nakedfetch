use crate::native::kusd::kuser_share_data;

/// REF: <https://ntdoc.m417z.com/kuser_shared_data>
#[must_use]
pub fn GetTickCount() -> u128 {
    let kusd_ref = kuser_share_data();
    let mult = u128::from(kusd_ref.tick_count_multiplier);
    let count = u128::from(kusd_ref.tick_count.quad());
    (mult * count) >> 0x18
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_tick_count_test() {
        unsafe extern "system" {
            safe fn GetTickCount64() -> u64;
        }

        let winapi = u128::from(GetTickCount64());
        let my = super::GetTickCount();
        let diff = winapi.abs_diff(my);

        assert!(diff <= 100, "Diff >100 ms, Winapi: {winapi}, My: {my}");
    }
}
