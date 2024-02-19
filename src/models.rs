#[derive(sqlx::FromRow)]
pub struct Item {
    pub item_id: i32,
    pub table_id: i32,
    pub cook_time: i32,   
}

#[derive(sqlx::FromRow)]
pub struct Table {
    pub table_id: i32,
}



