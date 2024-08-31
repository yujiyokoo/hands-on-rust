use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    let offset = Point::new(camera.left_x, camera.top_y);

    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)| {
            draw_batch.set(*pos - offset, render.colour, render.glyph);
        });
    draw_batch.submit(5000).expect("Batch error");
}
