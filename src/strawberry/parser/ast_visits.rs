use crate::strawberry::parser::ast::*;

pub trait Rule {}

impl Rule for Script {}

impl Rule for Declaration {}

impl Rule for FnPublicity {}

impl Rule for State {}

impl Rule for Member {}

impl Rule for Expression {}

impl Rule for Identifier {}

impl Rule for Literals {}
