/// Flipbook Timeline - plugin resmi ProCraft
/// Pakai kit resmi, hook position

static CONTROLS: kit::Controls<1> = kit::Controls::new([0.0]);
kit::controls!(CONTROLS);

// Hook position: geser posisi stroke pakai slider Across
kit::position!(|x, y| (x + CONTROLS.get(0), y));
