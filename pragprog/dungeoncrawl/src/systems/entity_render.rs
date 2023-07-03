use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);
    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)| {
            let pos = *pos - offset;
            let color = render.color;
            let glyph = render.glyph;
            draw_batch.set(pos, color, glyph);
        });
    draw_batch.submit(5000).expect("batch error");
}
