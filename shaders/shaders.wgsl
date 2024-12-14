
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(0)color: vec3<f32>,
}


@vertex
fn vertex(model: VertexInput) -> VertexOut {
    var out: VertexOut;
    out.color = model.color;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out; 
}

@fragment
fn fragment(in: VertexOut) -> @location(0) vec4<f32> {

    return vec4<f32>(in.color, 1.0) ;

}