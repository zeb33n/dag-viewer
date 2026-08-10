use crate::data_types::*;

pub struct Colours {
    pub node: Colour,
    pub edge: Colour,
    pub text: Colour,
}

pub struct ColoursHighlighted {
    pub node_hover: Colour,
    pub node_upstream: Colour,
    pub node_downstream: Colour,
    pub edge_upstream: Colour,
    pub edge_downstream: Colour,
    pub text: Colour,
}

pub const COLOURS: Colours = Colours {
    // Neutral gray background state
    node: 0x37415155,
    edge: 0xD1D5DB55,
    text: 0x37415135,
};

pub const COLOURS_HIGHLIGHT: ColoursHighlighted = ColoursHighlighted {
    node_hover: 0x7C3AEDFF,
    node_upstream: 0x2563EBFF,
    edge_upstream: 0x60A5FAFF,
    node_downstream: 0xDB2777FF,
    edge_downstream: 0xF472B6FF,
    text: 0x000000FF,
};
