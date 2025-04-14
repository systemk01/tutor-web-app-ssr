DROP table IF EXISTS ezyweb_user;

CREATE TABLE ezyweb_user
(
    username VARCHAR(20) primary key,
    tutor_id int,
    user_password char(100) not null
);
