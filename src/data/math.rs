pub fn reduction(v: f32, p: f32, b: f32) -> f32 {
    if (v*p).abs()<b {
        0.0
    }else{
        v*p
    }
}