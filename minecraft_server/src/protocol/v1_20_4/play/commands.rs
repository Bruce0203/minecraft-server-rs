use std::io::Result;

use crate::io::prelude::{Buffer, Decoder, Encoder, Identifier, VarString};

#[derive(Debug)]
pub struct Commands {
    nodes: Vec<Node>,
    root_index: i32,
}

impl Encoder for Commands {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        todo!()
    }
}

impl Decoder for Commands {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        //TODO wip
        Ok(Commands {
            nodes: vec![],
            root_index: 0,
        })
    }
}

#[derive(Debug)]
pub enum Node {
    Root(Root),
    Literal(Literal),
    Argument(Argument),
}

#[derive(Debug)]
pub struct Root {
    children: Vec<i32>,
    redirect_node: Option<i32>,
    name: Option<VarString<32767>>,
    parser_id: Option<i32>,
    properties: Vec<Parser>,
    suggestions_type: Identifier,
}

#[derive(Debug)]
pub enum Parser {
    Bool,
    Float,
    Double,
    Integer,
    Long,
    String,
    Entity,
    GameProfile,
    BlockPos,
    ColumnPos,
    Vec3,
    Vec2,
    BlockState,
    BlockPredicate,
    ItemStack,
    //TODO wip
}

#[derive(Debug)]
pub struct Literal {}

#[derive(Debug)]
pub struct Argument {}
