#[cfg(kani)]
#[kani::proof]
fn stone_flop_equality() {
    let stone: magpie::othello::Stone = kani::any();
    assert_eq!(stone, stone.flip().flip());
}
