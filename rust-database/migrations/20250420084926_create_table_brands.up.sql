-- Add up migration script here
create table brands
(
    id   serial primary key,
    name varchar(100) not null
);