drop table if exists ezyweb_user;

/* Create tables. */
/* Note: Don't put a comma after last field */
create table ezyweb_user
(
    username varchar(20) primary key,
    user_password VARCHAR(255) not null,
    tutor_id INT
);
