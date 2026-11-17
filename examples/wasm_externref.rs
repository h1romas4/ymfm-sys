#[cfg(all(
    target_arch = "wasm32",
    target_os = "unknown",
    target_vendor = "unknown"
))]
mod wasm {
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;
    use ymfm_sys::{ChipPtr, ffi};

    const CHIP_TYPES: &[ffi::ChipType] = &[
        ffi::ChipType::Ym2149,
        ffi::ChipType::Ym2151,
        ffi::ChipType::Ym2164,
        ffi::ChipType::Ym2203,
        ffi::ChipType::Ym2413,
        ffi::ChipType::Ym2423,
        ffi::ChipType::Ym2608,
        ffi::ChipType::Ym2610,
        ffi::ChipType::Ym2610B,
        ffi::ChipType::Ym2612,
        ffi::ChipType::Ym3438,
        ffi::ChipType::Ymf276,
        ffi::ChipType::Ym3526,
        ffi::ChipType::Ym3533,
        ffi::ChipType::Y8950,
        ffi::ChipType::Ym3812,
        ffi::ChipType::Ymf262,
        ffi::ChipType::Ymf281,
        ffi::ChipType::Ymf289B,
        ffi::ChipType::Ymf278B,
        ffi::ChipType::Ymf288,
        ffi::ChipType::Ym3806,
        ffi::ChipType::Ds1001,
        ffi::ChipType::Ym2414,
    ];
    const MAX_FRAMES: u32 = 65_536;

    struct Instance {
        handle: JsValue,
        chip: ChipPtr,
        samples: Vec<i32>,
    }

    thread_local! {
        static INSTANCES: RefCell<Vec<Instance>> = const { RefCell::new(Vec::new()) };
    }

    #[wasm_bindgen]
    pub fn ymfm_create(chip_type: u32, clock: u32) -> JsValue {
        let Some(&chip_type) = CHIP_TYPES.get(chip_type as usize) else {
            return JsValue::NULL;
        };
        if clock == 0 {
            return JsValue::NULL;
        }
        let handle: JsValue = js_sys::Object::new().into();
        let chip = ffi::create_chip(chip_type, clock);
        INSTANCES.with(|instances| {
            instances.borrow_mut().push(Instance {
                handle: handle.clone(),
                chip,
                samples: Vec::new(),
            });
        });
        handle
    }

    #[wasm_bindgen]
    pub fn ymfm_destroy(handle: &JsValue) -> i32 {
        INSTANCES.with(|instances| {
            let mut instances = instances.borrow_mut();
            match instances
                .iter()
                .position(|instance| &instance.handle == handle)
            {
                Some(index) => {
                    instances.swap_remove(index);
                    0
                }
                None => -1,
            }
        })
    }

    #[wasm_bindgen]
    pub fn ymfm_reset(handle: &JsValue) -> i32 {
        INSTANCES.with(|instances| {
            let mut instances = instances.borrow_mut();
            let Some(instance) = instances
                .iter_mut()
                .find(|instance| &instance.handle == handle)
            else {
                return -1;
            };
            instance.chip.pin_mut().reset();
            0
        })
    }

    #[wasm_bindgen]
    pub fn ymfm_write(handle: &JsValue, offset: u32, data: u8) -> i32 {
        INSTANCES.with(|instances| {
            let mut instances = instances.borrow_mut();
            let Some(instance) = instances
                .iter_mut()
                .find(|instance| &instance.handle == handle)
            else {
                return -1;
            };
            instance.chip.pin_mut().write(offset, data);
            0
        })
    }

    #[wasm_bindgen]
    pub fn ymfm_read(handle: &JsValue, offset: u32) -> i32 {
        INSTANCES.with(|instances| {
            let mut instances = instances.borrow_mut();
            match instances
                .iter_mut()
                .find(|instance| &instance.handle == handle)
            {
                Some(instance) => i32::from(instance.chip.pin_mut().read(offset)),
                None => -1,
            }
        })
    }

    #[wasm_bindgen]
    pub fn ymfm_channels(handle: &JsValue) -> i32 {
        INSTANCES.with(|instances| {
            match instances
                .borrow()
                .iter()
                .find(|instance| &instance.handle == handle)
            {
                Some(instance) => instance.chip.channels() as i32,
                None => -1,
            }
        })
    }

    #[wasm_bindgen]
    pub fn ymfm_sample_rate(handle: &JsValue) -> i64 {
        INSTANCES.with(|instances| {
            match instances
                .borrow()
                .iter()
                .find(|instance| &instance.handle == handle)
            {
                Some(instance) => i64::from(instance.chip.sample_rate()),
                None => -1,
            }
        })
    }

    #[wasm_bindgen]
    pub fn ymfm_memory() -> JsValue {
        wasm_bindgen::memory()
    }

    #[wasm_bindgen]
    pub fn ymfm_generate(handle: &JsValue, frames: u32) -> u32 {
        if frames == 0 || frames > MAX_FRAMES {
            return 0;
        }
        INSTANCES.with(|instances| {
            let mut instances = instances.borrow_mut();
            let Some(instance) = instances
                .iter_mut()
                .find(|instance| &instance.handle == handle)
            else {
                return 0;
            };
            let Some(len) = (frames as usize).checked_mul(instance.chip.channels() as usize) else {
                return 0;
            };
            if len > instance.samples.len()
                && instance
                    .samples
                    .try_reserve_exact(len - instance.samples.len())
                    .is_err()
            {
                return 0;
            }
            instance.samples.resize(len, 0);
            instance.chip.pin_mut().generate(&mut instance.samples);
            instance.samples.as_ptr() as u32
        })
    }
}

fn main() {}
