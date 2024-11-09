pub mod types {
    use std::rc::Rc;
    use crate::engine::script::env::Frame;
    pub enum Query {
        Select(Option<IdentList>, String, Option<Expr>, Option<String>, Option<Expr>), // SELECT _ FROM _ WHERE _ SORT BY _ LIMIT _ (where, sort by, and limit are optional)
        Insert(String, Option<IdentList>, ExprList), // INSERT INTO _ (_, _, _)? VALUES (_, _, _)
        SelectAggregate(String, String), // SELECT AGGREGATE <name> FROM <table>
        Const(String, Expr), // CONST <name> = <value>
        Aggregate(String, Expr, String), // AGGREGATE <name> = <value> INTO <table>
        Column(String, Expr, String), // COLUMN <name> = <value> INTO <table>
        CreateTable(String, ColList), // CREATE TABLE <name> (col1 type1, col2type2, ...)
    }
    pub enum Expr {
        BopExpr(Rc<Expr>, BopType, Rc<Expr>),
        UopExpr(UopType, Rc<Expr>),
        BlockExpr(Block),
        ValExpr(Val),
        IdentExpr(String),
        CallExpr(Rc<Expr>, ExprList),
        FunExpr(IdentList, Rc<Expr>),
        CondExpr(Rc<Expr>, Rc<Expr>, Rc<Expr>) // if _ then _ else _
    }
    pub enum Block {
        ExprBlock(Rc<Expr>),
        StmtBlock(String, Rc<Expr>, Rc<Block>) // ident = expr; ...
    }
    #[derive(Clone)]
    pub enum Val {
        NumVal(f64),
        StrVal(String),
        BoolVal(bool),
        UndefVal,
        NullVal,
        ClosureVal(Frame, IdentList, Rc<Expr>)
    }
    #[derive(PartialEq, Debug)]
    pub enum BopType {
        PlusBop,
        MinusBop,
        TimesBop,
        DivBop,
        GtBop,
        GteBop,
        LtBop,
        LteBop,
        EqBop,
        StrEqBop,
        LogOrBop,
        LogAndBop
    }
    pub type ColList = Vec<(String, ColType)>;
    pub type ExprList = Vec<Rc<Expr>>;
    pub type IdentList = Vec<String>;
    #[derive(PartialEq, Debug)]
    pub enum UopType {
        NegUop,
        NotUop
    }
    #[derive(Clone)]
    pub enum ColType {
        Number,
        String,
        Boolean
    }
}

