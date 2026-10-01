pub struct Ctx {
    pub dt: f32,
}

impl Ctx {
    pub(crate) fn new() -> Self {
        Ctx { dt: 0.0 }
    }
}
