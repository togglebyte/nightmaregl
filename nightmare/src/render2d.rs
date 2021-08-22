struct Render2d {
    shader: ShaderProgram,
    vao: Vao,
}

impl Render2d {
//     fn new() -> Self {
//         Self {
//         }
//     }
}

struct Render<'a> {
    shader: &ShaderProgram,
    vao: &Vao,
}
