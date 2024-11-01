pub mod table {
    use super::super::column;
    struct Table {
        table: Vec<column::generic::ColumnContainer>,
        headers: Vec<String>
    }
}