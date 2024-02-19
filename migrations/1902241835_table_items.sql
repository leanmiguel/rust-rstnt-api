create table table_items (
    item_id int generated always as identity,
    table_id int not null,
    cook_time int not null,
    primary key (item_id),
    foreign key (table_id) references restaurant_tables(table_id)
);