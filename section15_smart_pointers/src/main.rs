mod box_and_heap;
mod cleanup;
mod my_box;
mod ref_counters;
mod ref_cell_interior_mutability;

fn main() {
    box_and_heap::box_heap();
    my_box::my_box();
    cleanup::cleanup();
    ref_counters::rc();
    ref_cell_interior_mutability::interior_mut();
}
